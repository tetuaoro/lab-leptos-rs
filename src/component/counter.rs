use crate::api::counter;

use leptos::*;

/// Renders the counter page of your application.
#[component]
pub fn CounterPage() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    let click_me_baby = move |_| {
        spawn_local(async move {
            if let Ok(c) = counter().await {
                set_count.update(|sc| *sc = c);
            }
        });
    };

    view! {
        <div>"Count signal : " {count}</div>
        <button on:click=click_me_baby>"Click me bb"</button>
    }
}
