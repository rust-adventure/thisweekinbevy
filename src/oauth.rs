use axum::{
    extract::{FromRequestParts, Query},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_login::{tower_sessions::Session, AuthnBackend};
use http::request::Parts;
use oauth2::CsrfToken;
use serde::Deserialize;

use crate::{
    auth::NEXT_URL_KEY,
    state::AppState,
    users::{AuthSession, Credentials},
};

pub const CSRF_STATE_KEY: &str = "oauth.csrf-state";

#[derive(Debug, Clone, Deserialize)]
pub struct AuthzResp {
    code: String,
    state: CsrfToken,
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route(
        "/oauth/callback",
        get(self::get::callback),
    )
}

mod get {
    use tracing::instrument;

    use super::*;

    #[axum::debug_handler]
    #[instrument(skip(auth_session))]
    pub async fn callback(
        mut auth_session: AuthSession,
        Query(AuthzResp {
            code,
            state: new_state,
        }): Query<AuthzResp>,
    ) -> impl IntoResponse {
        let Ok(Some(old_state)) =
            auth_session.session.get(CSRF_STATE_KEY).await
        else {
            return StatusCode::BAD_REQUEST.into_response();
        };

        let creds = Credentials {
            code,
            old_state,
            new_state,
        };

        let user = match auth_session
            .authenticate(creds)
            .await
        {
            Ok(Some(user)) => user,
            Ok(None) => {
                return (
                    StatusCode::UNAUTHORIZED,
                    "Invalid CSRF state", /* LoginTemplate {
                                           *     message: Some(
                                           *         "Invalid CSRF state."
                                           *
                                           * .to_string(),
                                           *
                                           *     ),
                                           *     next: None,
                                           * }, */
                )
                    .into_response();
            }
            Err(_) => {
                return StatusCode::INTERNAL_SERVER_ERROR
                    .into_response()
            }
        };

        if auth_session.login(&user).await.is_err() {
            return StatusCode::INTERNAL_SERVER_ERROR
                .into_response();
        }

        if let Ok(Some(next)) = auth_session
            .session
            .remove::<String>(NEXT_URL_KEY)
            .await
        {
            Redirect::to(&next).into_response()
        } else {
            Redirect::to("/").into_response()
        }
    }
}
