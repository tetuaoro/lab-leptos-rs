use leptos::*;
use leptos_router::use_params_map;

/// Renders the slug page of your application.
#[component]
pub fn SlugPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned());

    view! {
        <h1>"Slug Page"</h1>
        <p>{slug}</p>
    }
}

/// Renders the project page of your application.
#[component]
pub fn ProjectPage() -> impl IntoView {
    let params = use_params_map();
    let slug = move || params.with(|params| params.get("slug").cloned());
    let project = move || params.with(|params| params.get("project").cloned());

    view! {
        <h1>"Project Page"</h1>
        <p>{slug}</p>
        <p>{project}</p>
    }
}
