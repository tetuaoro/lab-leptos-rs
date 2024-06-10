#![allow(unused)]

use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::i18n::*;

use ev::MouseEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::time::Duration;
use web_sys::HtmlButtonElement;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // Provides context that manages languages.
    provide_i18n_context();

    let fallback_error = || {
        let mut outside_errors = Errors::default();
        outside_errors.insert_with_default_key(AppError::NotFound);
        view! { <ErrorTemplate outside_errors/> }.into_view()
    };

    view! {
        <Title text="Welcome to Leptos"/>
        <Stylesheet id="leptos" href="/pkg/hexagonal-arch.css"/>
        <Router fallback=fallback_error>
            <main>
                <Routes>
                    <Route path="/" view=|| view! { <Redirect path="/en"/> }/>
                    <Route path="/:lang" view=LangOutlet>
                        <Route path="" view=HomePage/>
                        <Route path="clock" view=Clock/>
                        <Route path="locatorjs" view=LocatorJsPage/>
                        <Route path="slice-signal" view=SliceSignalPage/>
                        <Route path="console-log" view=ConsoleLogPage/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let routes = ["clock", "locatorjs", "slice-signal", "console-log"];

    use leptonic::prelude::*;

    view! {
        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.6)>
            <Skeleton animated=false/>
            <Skeleton animated=false>
                <div>
                    <h3>"Pages"</h3>
                    <ul style="list-style:none;padding:0;text-align:start;">
                        <For each=move || routes key=|r| r.to_owned() let:route>
                            <li>
                                <A href=route>{route}</A>
                            </li>
                        </For>
                    </ul>
                </div>
            </Skeleton>
            <Skeleton animated=false/>
        </Stack>
    }
}

impl<'a> Into<&'a str> for Locale {
    fn into(self) -> &'a str {
        match self {
            Locale::en => "en",
            Locale::fr => "fr",
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
        None
    }
}

#[component]
fn LangOutlet() -> impl IntoView {
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

    let style = "display:flex;justify-content:center;gap:3px;";

    view! {
        <div style="margin-bottom:20px;">
            <h2>"set language"</h2>
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
                <A href="/">{t!(i18n, fare)}</A>
            </div>
        </div>
        <Outlet/>
    }
}
