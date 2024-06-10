use crate::i18n::*;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Clock() -> impl IntoView {
    let i18n = use_i18n();
    let lang: &str = i18n.get_locale().into();

    view! {
        <div>
            <p>"theres some stuff here that doesnt matter"</p>
            <p>{t!(i18n, te_huru)}</p>
            <A href=format!("/{lang}")>{t!(i18n, fare)}</A>
        </div>
    }
}
