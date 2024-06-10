use crate::error_template::{AppError, ErrorTemplate};
use crate::i18n::*;

use crate::components::*;
use ev::MouseEvent;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use web_sys::HtmlButtonElement;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_i18n_context();

    view! {
        <Title text="Welcome to Leptos"/>
        <Stylesheet id="leptos" href="/pkg/hexagonal-arch.css"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <body>
                <main>
                    <Routes>
                        <Route path="" view=move || view! { <Redirect path="/en"/> }/>
                        <Route path=":lang" view=LangOutlet>
                            <Route path="" view=HomePage/>
                            <Route path="clock" view=Clock/>
                            <Route path="locatorjs" view=LocatorJsPage/>
                            <Route path="slice-signal" view=SliceSignalPage/>
                            <Route path="console-log" view=ConsoleLogPage/>
                        </Route>
                    </Routes>
                </main>
            </body>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let style = r"
        width:100%;
        height:100%;
        display:flex;
        flex-direction:column;
        justify-content:center;
        gap: 3px;
    ";

    let routes = ["clock", "locatorjs", "slice-signal", "console-log"];

    view! {
        <div style=style>
            <For each=move || routes key=|r| r.to_owned() let:route>
                <A href=route>{format!("{route} page")}</A>
            </For>
        </div>
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

        let to_locale: &str = current_locale.into();

        let pathname = location.pathname.get();
        let pathname = pathname
            .split("/")
            .into_iter()
            .enumerate()
            .map(|(index, s)| {
                if index == 1 {
                    return to_locale;
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

    let style = r"
        display: flex;
        justify-content: center;
        gap: 3px;
    ";

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
                <button on:click=handle_locale attr:data-lang="d">
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
