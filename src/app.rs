use crate::alert::ModalProvider;
use crate::component::slug::{ProjectPage, SlugPage};
use crate::component::Page;
use crate::error_template::{AppError, ErrorTemplate};
use crate::i18n::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

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
                            <Route path="/:lang" view=Outlet>
                                <Route path="" view=Page/>
                                <Route path=":slug" view=SlugPage/>
                                <Route path=":slug/:project" view=ProjectPage/>
                                <Route path=":slug/:project/train" view=Page/>
                            </Route>
                        </Routes>
                    </main>
                </body>
            </ModalProvider>
        </Router>
    }
}
