use crate::api::counter;

use leptos::*;

/// Renders the counter page of your application.
#[component]
pub fn CounterPage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let (message, set_message) = create_signal("My message 1");

    let click_me_baby = move |_| {
        spawn_local(async move {
            if let Ok(c) = counter().await {
                set_count.update(|sc| *sc = c);
            }
        });

        set_message.update(|sm| *sm = "My 22 message");
    };

    view! {
        <div>"Count signal : " {count}</div>
        <div>"Message : " {message}</div>
        <button on:click=click_me_baby>"Click me bb"</button>
    }
}
