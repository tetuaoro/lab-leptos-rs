use crate::alert::ModalProvider;
use crate::error_template::{AppError, ErrorTemplate};
use crate::i18n::*;

use ev::{Event, MouseEvent};
use html::Input;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::ops::Add;
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
            <ModalProvider>
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
            </ModalProvider>
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

#[derive(Debug, Default, Clone)]
struct SliceSignalStruct {
    names: Vec<String>,
    how_many_in_your_life: u8, // u0 for me xD
}

impl SliceSignalStruct {
    fn with(surname: String) -> Self {
        Self {
            names: vec![surname],
            how_many_in_your_life: Default::default(),
        }
    }
}

#[component]
fn SliceSignalPage() -> impl IntoView {
    let input_ref = create_node_ref::<Input>();
    let the_first_love = SliceSignalStruct::with("Babe".into());
    let signal = create_rw_signal(the_first_love);

    let getter = |signal: &SliceSignalStruct| signal.names.clone();
    let setter = move |state: &mut SliceSignalStruct, new_value| {
        state.names.push(new_value);
        let _ = state.how_many_in_your_life.add(1);

        if let Some(elm) = input_ref.get() {
            let _ = elm.focus();
        }
    };
    let (surnames, set_surnames) = create_slice(signal, getter, setter);

    let on_input = move |evt: Event| {
        let value = event_target_value(&evt);
        set_surnames.set(value);
    };

    view! {
        <label for="babies">"What's her surname :"</label>
        <input node_ref=input_ref id="babies" on:input=on_input/>
        <ul>
            <For each=move || surnames.get() key=|surname| surname.clone() let:surname>
                <li>{surname}</li>
            </For>
        </ul>
    }
}

#[component]
fn ConsoleLogPage() -> impl IntoView {
    let initial_title = String::from("Hello, world.");
    let (title, set_title) = create_signal(initial_title);

    leptos::logging::log!("HomePage component");

    view! {
        <form method="GET" action="">
            <label>
                "I want to read:"
                <input
                    type="text"
                    prop:value=title
                    on:input=move |evt| {
                        let input = event_target_value(&evt);
                        leptos::logging::log!("logging Input: {input}");
                        set_title.set(input);
                    }
                />

            </label>
            <button>"Create"</button>
        </form>
    }
}

#[server]
async fn pray_me() -> Result<(), ServerFnError> {
    use std::thread::sleep;
    use std::time::Duration;

    sleep(Duration::from_secs(5));
    Ok(())
}

#[component]
#[leptos_locatorjs::add_locatorjs_id]
pub fn LocatorJsPage() -> impl IntoView {
    let (count, _) = create_signal(2);
    let ressource = create_resource(|| (), |_| async move { pray_me().await });

    let hello_word = move || {
        let my_count = count.get();
        match my_count {
            2 => view! { <h2>"Hello, world!"</h2> },
            _ => view! { <h2>"Burn, world!"</h2> },
        }
    };

    let god____where_r_u = move || {
        let _son_______i_am_everywhere = ressource.get();
        "Je suis là, mon fils"
    };

    view! {
        <div>
            <div>{hello_word}</div>
            <Suspense fallback=|| view! { <div>"Loading..."</div> }>
                <ul>
                    <li>"I like banana."</li>
                    <li>{god____where_r_u}</li>
                </ul>
            </Suspense>
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
