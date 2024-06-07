mod adapter;
mod alert;
mod api;
pub mod app;
mod component;
mod entity;
mod error_template;

#[cfg(feature = "ssr")]
pub use tracing;

#[cfg(feature = "ssr")]
pub mod fileserv;
leptos_i18n::load_locales!();

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
