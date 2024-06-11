use crate::i18n::*;
use ev::MouseEvent;
use leptos::*;
use leptos_router::*;
use web_sys::HtmlButtonElement;

/* 3 things : into impl, from impl & view! macro */

impl<'a> Into<&'a str> for Locale {
    fn into(self) -> &'a str {
        match self {
            Locale::en => "en",
            Locale::fr => "fr",
            Locale::ty => "ty",
        }
    }
}

impl Locale {
    fn from(value: String) -> Option<Self> {
        if value.eq("fr") {
            return Some(Self::fr);
        }
        if value.eq("en") {
            return Some(Self::en);
        }
        if value.eq("ty") {
            return Some(Self::ty);
        }
        None
    }
}

#[component]
pub fn LangOutlet() -> impl IntoView {
    let i18n = use_i18n();
    let navigate = use_navigate();
    let location = use_location();

    let (locale, set_locale) = create_signal(Locale::en);

    create_effect(move |from_locale: Option<Locale>| {
        let current_locale = locale.get();

        if let Some(from_locale) = from_locale {
            if from_locale.eq(&current_locale) {
                return from_locale;
            }
        }

        i18n.set_locale(current_locale);

        let pathname = location.pathname.get();
        let pathname = pathname
            .split("/")
            .into_iter()
            .enumerate()
            .map(|(index, s)| {
                if index == 1 {
                    return current_locale.into();
                }
                s
            })
            .collect::<Vec<_>>()
            .join("/");

        navigate(&pathname, Default::default());

        locale.get()
    });

    let handle_locale = move |evt: MouseEvent| {
        let elm = event_target::<HtmlButtonElement>(&evt);
        if let Some(lang) = elm.get_attribute("data-lang") {
            if let Some(locale) = Locale::from(lang) {
                set_locale.set(locale);
            }
        }
    };

    let home_href = move || {
        let lang: &str = locale.get().into();
        format!("/{lang}")
    };

    let style = "display:flex;justify-content:center;gap:3px;";

    view! {
        <div style="margin-bottom:20px;">
            <h2>"Set locale language"</h2>
            <div style=style>
                <button on:click=handle_locale attr:data-lang="en">
                    "en"
                </button>
                <button on:click=handle_locale attr:data-lang="fr">
                    "fr"
                </button>
                <button on:click=handle_locale attr:data-lang="ty">
                    "ty"
                </button>
            </div>
            <div>
                <br/>
                <A href=home_href>{t!(i18n, fare)}</A>
            </div>
        </div>
        <Outlet/>
    }
}
