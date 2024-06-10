use crate::i18n::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Clock() -> impl IntoView {
    let i18n = use_i18n();

    let get_locale = move || {
        let locale = i18n.get_locale();
        let lang: &str = locale.into();
        lang
    };

    view! {
        <div>
            <p>"theres some stuff here that doesnt matter"</p>
            <p>{t!(i18n, te_huru)}</p>
            <A href=move || format!("/{}", get_locale())>{t!(i18n, fare)}</A>
        </div>
    }
}
