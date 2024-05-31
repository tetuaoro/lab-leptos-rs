use crate::alert::ModalProvider;
use crate::component::Page;
use crate::error_template::{AppError, ErrorTemplate};
use crate::i18n::*;
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

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    provide_i18n_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/hexagonal-arch.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <ModalProvider>
                <body>
                    <main>
                        <Routes>
                            <Route path="/" view=move || view! { <Redirect path="/en"/> }/>
                            <Route path="/:lang" view=LangOutlet>
                                <Route path="" view=Page/>
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
    let get_lang = move || {
        params.with_untracked(|paramap| {
            if let Some(lang) = paramap.get("lang").cloned() {
                if lang.eq("en") {
                    return Some(Locale::en);
                }
                if lang.eq("fr") {
                    return Some(Locale::fr);
                }
            }
            None
        })
    };
    let lang = get_lang().map(|lang| lang);

    let i18n = use_i18n();
    if let Some(lang) = lang {
        i18n.set_locale(lang);
    }

    view! {
        <Show when=move || Option::is_some(&lang) fallback=|| view! { <Redirect path="/en"/> }>
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
            <A href={format!("/{lang}")}>{t!(i18n, fare)}</A>
        </div>
    }
}
