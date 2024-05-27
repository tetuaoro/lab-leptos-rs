mod counter;
mod home;
mod modal;
mod moffu;
pub mod slug;
mod user;

use counter::CounterPage;
use home::HomePage;
use leptos_router::use_params_map;
use modal::ModalPage;
use moffu::MoffuPage;
// use user::UserPage;

use crate::i18n::*;
use leptos::*;

#[component]
pub fn Page() -> impl IntoView {
    let params = use_params_map();
    let lang = move || {
        params.with(|params| {
            if let Some(lang) = params.get("lang").cloned() {
                if lang.contains("fr") {
                    return Locale::fr;
                }
            }
            Locale::default()
        })
    };

    let i18n = use_i18n();
    i18n.set_locale(lang());

    view! {
        <HomePage/>
        <CounterPage/>
        <MoffuPage/>
        <ModalPage/>

        <p>{t!(i18n, te_huru)}</p>

        <div id="app" style="margin:20px">
            "App"
        </div>
        <div id="app1">"App1"</div>
        <div id="app2">"App2"</div>
    }
}
