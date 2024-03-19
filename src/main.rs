use axum::{
    body::Body as AxumBody,
    extract::{Path, State},
    http::Request,
    response::{IntoResponse, Response},
    routing::get,
};
use axum_login::{
    tower_sessions::{
        cookie::SameSite, Expiry, SessionManagerLayer,
    },
    AuthManagerLayerBuilder,
};
use leptos::provide_context;
use leptos_axum::handle_server_fns_with_context;
use oauth2::{
    basic::BasicClient, AuthUrl, ClientId, ClientSecret,
    TokenUrl,
};
use sqlx::mysql::MySqlPoolOptions;
use std::env;
use this_week_in_bevy::{
    app::App, auth, oauth, state::AppState, users::Backend,
};
use this_week_in_bevy::{
    session_store, users::AuthSession, Username,
};
use time::Duration;

#[tracing::instrument(skip(
    app_state,
    auth_session,
    request
))]
async fn server_fn_handler(
    State(app_state): State<AppState>,
    path: Path<String>,
    auth_session: AuthSession,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.pool.clone());
            provide_context(
                auth_session.user.as_ref().map(|user| {
                    Username(user.username.clone())
                }),
            );
        },
        request,
    )
    .await
}

async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    auth_session: AuthSession,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(app_state.pool.clone());
            provide_context(
                auth_session.user.as_ref().map(|user| {
                    Username(user.username.clone())
                }),
            );
        },
        App,
    );
    handler(req).await.into_response()
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use this_week_in_bevy::app::*;
    use this_week_in_bevy::fileserv::file_and_error_handler;

    tracing_subscriber::fmt::init();

    let pool = MySqlPoolOptions::new()
        .connect(
            &env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
        )
        .await
        .expect("Could not make pool.");

    let client_id = env::var("GITHUB_CLIENT_ID")
        .map(ClientId::new)
        .expect("GITHUB_CLIENT_ID should be provided.");
    let client_secret = env::var("GITHUB_CLIENT_SECRET")
        .map(ClientSecret::new)
        .expect("GITHUB_CLIENT_SECRET should be provided");

    let auth_url = AuthUrl::new(
        "https://github.com/login/oauth/authorize"
            .to_string(),
    )
    .expect("it to have worked :: authorize");
    let token_url = TokenUrl::new(
        "https://github.com/login/oauth/access_token"
            .to_string(),
    )
    .expect("it to have worked :: access_token");
    let client = BasicClient::new(
        client_id,
        Some(client_secret),
        auth_url,
        Some(token_url),
    );

    // Session layer.
    //
    // This uses `tower-sessions` to establish a layer
    // that will provide the session as a request
    // extension.
    let session_store =
        session_store::MySqlStore::new(pool.clone());
    // a memory store would be instantiated like this
    // let session_store = MemoryStore::default();

    // migrations happen in `bin/migrate-sessions` as
    // we don't do "live migrations".
    // session_store.migrate().await?;

    let session_layer =
        SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_same_site(SameSite::Lax) // Ensure we send the cookie from the OAuth
            // redirect.
            .with_expiry(Expiry::OnInactivity(
                Duration::days(1),
            ));

    // Auth service.
    //
    // This combines the session layer with our
    // backend to establish the auth service which
    // will provide the auth session as a request
    // extension.
    let backend = Backend::new(pool.clone(), client);
    let auth_layer = AuthManagerLayerBuilder::new(
        backend,
        session_layer,
    )
    .build();

    // Setting get_configuration(None) means we'll be
    // using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as
    // Some("Cargo.toml") The file would need to
    // be included with the executable when moved to
    // deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
        routes: routes.clone(),
    };

    // build our application with a route
    let app = Router::new()
        .route("/feed.xml", get(this_week_in_bevy::atom_feed::atom_feed))
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(
            routes,
            get(leptos_routes_handler),
        )
        // .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .merge(auth::router())
        .merge(oauth::router())
        .layer(auth_layer)
        .with_state(app_state);

    let listener =
        tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g.,
    // Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
