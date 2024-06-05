use crate::alert::ModalProvider;
use crate::component::*;
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
            <ModalProvider>
                <body>
                    <main>
                        <Routes>
                        <Route path="" view=GridPage/>
                            // <Route path="/" view=move || view! { <Redirect path="/en"/> }/>
                            // <Route path="/:lang" view=LangOutlet>
                            //     <Route path="" view=Page/>
                            //     <Route path="clock" view=Clock/>
                            // </Route>
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
    let navigate = use_navigate();

    let get_lang = params.with_untracked(|paramap| {
        if let Some(value) = paramap.get("lang").cloned() {
            return Locale::from(value);
        }
        None
    });

    if let Some(lang) = get_lang {
        let lang_str: &str = lang.into();
        let new_route = format!("/{}", lang_str);

        i18n.set_locale(lang);
        navigate(new_route.as_str(), Default::default());
    }

    let location = window().location();
    let _hash = location.hash();
    let location = use_location();
    let _hash = move || location.hash.get();

    view! {
        <div>
            <fieldset>
                <h2>"set language"</h2>
                <a href="/en">"en"</a>
                <span>" "</span>
                <a href="/fr">"fr"</a>
            </fieldset>
        </div>
        <Show when=move || Option::is_some(&get_lang) fallback=|| view! { <Redirect path="/en"/> }>
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
