use async_trait::async_trait;
use axum::http::header::{AUTHORIZATION, USER_AGENT};
use axum_login::{AuthUser, AuthnBackend, UserId};
use oauth2::{
    basic::{BasicClient, BasicRequestTokenError},
    reqwest::{async_http_client, AsyncHttpClientError},
    url::Url,
    AuthorizationCode, CsrfToken, TokenResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use tracing::{error, info, instrument};

#[derive(Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub github_id: String,
    pub username: String,
    pub access_token: String,
}

// Here we've implemented `Debug` manually to
// avoid accidentally logging the access token.
impl std::fmt::Debug for User {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.debug_struct("User")
            // .field("github_id", &self.github_id)
            .field("username", &self.username)
            .field("access_token", &"[redacted]")
            .finish()
    }
}

impl AuthUser for User {
    type Id = String;

    fn id(&self) -> Self::Id {
        self.github_id.to_string()
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.access_token.as_bytes()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub code: String,
    pub old_state: CsrfToken,
    pub new_state: CsrfToken,
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    id: u64,
    login: String,
}

#[derive(Debug, Deserialize)]
struct UserInfoFromDb {
    id: String,
    login: String,
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error(transparent)]
    Sqlx(sqlx::Error),

    #[error(transparent)]
    Reqwest(reqwest::Error),

    #[error(transparent)]
    OAuth2(BasicRequestTokenError<AsyncHttpClientError>),
}

#[derive(Debug, Clone)]
pub struct Backend {
    db: MySqlPool,
    client: BasicClient,
}

impl Backend {
    #[instrument(skip(db, client))]
    pub fn new(db: MySqlPool, client: BasicClient) -> Self {
        Self { db, client }
    }
    #[instrument(skip(self))]
    pub fn authorize_url(&self) -> (Url, CsrfToken) {
        self.client
            .authorize_url(CsrfToken::new_random)
            .url()
    }
}

#[async_trait]
impl AuthnBackend for Backend {
    type User = User;
    type Credentials = Credentials;
    type Error = BackendError;

    #[instrument(skip(self, creds), err)]
    async fn authenticate(
        &self,
        creds: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        // Ensure the CSRF state has not been tampered
        // with.
        if creds.old_state.secret()
            != creds.new_state.secret()
        {
            return Ok(None);
        };

        // Process authorization code, expecting a token
        // response back.
        let token_res = self
            .client
            .exchange_code(AuthorizationCode::new(
                creds.code,
            ))
            .request_async(async_http_client)
            .await
            .inspect_err(|v| {
                error!(?v);
            })
            .map_err(Self::Error::OAuth2)?;

        // Use access token to request user info.
        let user_info = reqwest::Client::new()
            .get("https://api.github.com/user")
            .header(
                USER_AGENT.as_str(),
                "this-week-in-bevy",
            ) // See: https://docs.github.com/en/rest/overview/resources-in-the-rest-api?apiVersion=2022-11-28#user-agent-required
            .header(
                AUTHORIZATION.as_str(),
                format!(
                    "Bearer {}",
                    token_res.access_token().secret()
                ),
            )
            .send()
            .await
            .map_err(Self::Error::Reqwest)?
            .json::<UserInfo>()
            .await
            .map_err(Self::Error::Reqwest)?;

        // Persist user in our database so we can use
        // `get_user`.
        let _num_rows_affected = sqlx::query(
            r#"
            insert into github_users (github_id, username, access_token)
            values (?, ?, ?)
            ON DUPLICATE KEY
            UPDATE access_token = ?;
            "#,
        )
        .bind(&user_info.id.to_string())
        .bind(&user_info.login)
        .bind(token_res.access_token().secret())
        .bind(token_res.access_token().secret())
        .execute(&self.db)
        .await
        .map_err(Self::Error::Sqlx)?;

        let user = sqlx::query_as(
            r#"
        SELECT * FROM github_users
        WHERE username = ?;"#,
        )
        .bind(user_info.login)
        .fetch_one(&self.db)
        .await
        .map_err(Self::Error::Sqlx)?;
        Ok(Some(user))
    }

    #[instrument(skip(self))]
    async fn get_user(
        &self,
        user_id: &UserId<Self>,
    ) -> Result<Option<Self::User>, Self::Error> {
        Ok(sqlx::query_as(
            "select * from github_users where github_id = ?",
        )
        .bind(user_id)
        .fetch_optional(&self.db)
        .await
        .map_err(Self::Error::Sqlx)?)
    }
}

// We use a type alias for convenience.
//
// Note that we've supplied our concrete backend
// here.
pub type AuthSession = axum_login::AuthSession<Backend>;
