use crate::components::*;
use crate::error_template::{AppError, ErrorTemplate};
use crate::i18n::*;
use crate::locale::LangOutlet;

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
        <Stylesheet id="leptos" href="/pkg/lab-leptos-rs.css"/>
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
                        <Route path="ctrl-shift" view=CtrlShiftKey/>
                        <Route path="height-child" view=HeightParent/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    let routes = [
        "clock",
        "locatorjs",
        "slice-signal",
        "console-log",
        "ctrl-shift",
        "height-child",
    ];

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
