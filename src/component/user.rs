use leptos::*;

use crate::adapter::UserAdapter;
use crate::api::get_users;

/// Renders the user page of your application.
#[component]
pub fn UserPage() -> impl IntoView {
    let users = create_resource(
        || (),
        |_| async move { get_users().await.map_or(vec![], |res| res) },
    );

    view! {
        <div>
            <h1>"User Page"</h1>
            <For each=move || users.get().unwrap() key=|u| u.get_username() let:user>
                <h2>{user.get_username()}</h2>
            </For>
        </div>
    }
}
