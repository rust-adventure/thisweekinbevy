pub mod app;
#[cfg(feature = "ssr")]
pub mod auth;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;
#[cfg(feature = "ssr")]
pub mod markdown;
#[cfg(feature = "ssr")]
pub mod oauth;
pub mod sql;
#[cfg(feature = "ssr")]
pub mod state;
#[cfg(feature = "ssr")]
pub mod users;
#[cfg(feature = "ssr")]
pub mod session_store;
pub mod issue_date;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Username(pub String);

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    // leptos::mount_to_body(App);
    leptos::leptos_dom::HydrationCtx::stop_hydrating();
}
