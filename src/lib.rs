pub mod app;
mod components;
mod error_template;
mod locale;
mod server_fn;

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
