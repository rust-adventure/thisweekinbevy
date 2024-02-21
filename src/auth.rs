use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use axum_login::tower_sessions::Session;
use serde::Deserialize;

use crate::{
    oauth::CSRF_STATE_KEY, state::AppState,
    users::AuthSession,
};

pub const NEXT_URL_KEY: &str = "auth.next-url";

// This allows us to extract the "next" field from
// the query string. We use this to redirect after
// log in.
#[derive(Debug, Deserialize)]
pub struct NextUrl {
    next: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/login", post(self::post::login))
        .route("/logout", get(self::get::logout))
}

mod post {
    use tracing::instrument;

    use super::*;

    #[instrument]
    pub async fn login(
        auth_session: AuthSession,
        session: Session,
        Form(NextUrl { next }): Form<NextUrl>,
    ) -> impl IntoResponse {
        let (auth_url, csrf_state) =
            auth_session.backend.authorize_url();

        session
            .insert(CSRF_STATE_KEY, csrf_state.secret())
            .await
            .expect("Serialization should not fail.");

        session
            .insert(NEXT_URL_KEY, next)
            .await
            .expect("Serialization should not fail.");

        Redirect::to(auth_url.as_str()).into_response()
    }
}

mod get {
    use tracing::instrument;

    use super::*;

    #[instrument]
    pub async fn logout(
        mut auth_session: AuthSession,
    ) -> impl IntoResponse {
        match auth_session.logout().await {
            Ok(_) => Redirect::to("/login").into_response(),
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR
                .into_response(),
        }
    }
}
