#![allow(unused)]

use std::thread::sleep;
use std::time::Duration;

use crate::alert::ModalProvider;
use crate::api::counter;
use crate::component::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::i18n::*;
use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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

#[server]
async fn pray_me() -> Result<(), ServerFnError> {
    sleep(Duration::from_secs(5));
    Ok(())
}

#[component]
#[leptos_locatorjs::add_locatorjs_id]
pub fn Example() -> impl IntoView {
    let (count, _) = create_signal(2);
    let ressource = create_resource(|| (), |_| async move { pray_me().await });

    let hello_word = move || {
        let my_count = count.get();
        match my_count {
            2 => view! {<div>"Hello, world!"</div>},
            _ => view! {<div>"Burn, world!"</div>},
        }
    };

    let god____where_r_u = move || {
        let _son_______i_am_everywhere = ressource.get();
        "Je suis là, mon fils"
    };

    view! {
        <div>
            <h1>{hello_word}</h1>
            <Suspense fallback=||view!{ <div>"Loading..."</div> }>
                <ul>
                    <li>"I like banana."</li>
                    <li>{god____where_r_u}</li>
                </ul>
            </Suspense>
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_i18n_context();

    #[cfg(feature = "ssr")]
    leptos::tracing::trace!("babe?");
    leptos::leptos_dom::tracing::trace!("babe?");
    #[cfg(feature = "ssr")]
    crate::tracing::trace!("babe?");

    view! {
        <Title text="Welcome to Leptos"/>
        <Stylesheet id="leptos" href="/pkg/hexagonal-arch.css"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <ModalProvider>
                <body>
                    <main>
                        <Routes>
                            <Route path="" view=move || view! { <Redirect path="/en"/> }/>
                            <Route path=":lang" view=LangOutlet>
                                <Route path="" view=Example/>
                                <Route path="clock" view=Clock/>
                            </Route>
                        </Routes>
                    </main>
                </body>
            </ModalProvider>
        </Router>
    }
}

#[component]
fn LangOutlet() -> impl IntoView {
    let params = use_params_map();
    let i18n = use_i18n();

    let get_lang = params.with_untracked(|paramap| {
        if let Some(value) = paramap.get("lang").cloned() {
            return Locale::from(value);
        }
        None
    });

    if let Some(lang) = get_lang {
        i18n.set_locale(lang);
        cfg_if! {
            if #[cfg(feature = "hydrate")] {
                let navigate = use_navigate();
                let lang_str: &str = lang.into();
                let new_route = format!("/{}", lang_str);
                navigate(new_route.as_str(), Default::default());
            }
        }
    }

    view! {
        <div>
            <fieldset>
                <h2>"set language"</h2>
                <a href="/en">"en"</a>
                <span>" "</span>
                <a href="/fr">"fr"</a>
                < a href="d" a>"ty"</a>
            </fieldset>
        </div>
        <Show when=move || Option::is_some(&get_lang) fallback=|| view! { <div></div> }>
            <Outlet/>
        </Show>
    }
}

#[component]
fn Clock() -> impl IntoView {
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
