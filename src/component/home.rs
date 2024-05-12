use crate::entity::Message;
use crate::adapter::PrintMessage;
use leptos::*;

#[component]
pub fn HomeWrapper() -> impl IntoView {
    let test = Message::from(String::from("test browzer"));

    view! {
        <HomePage message={&test} />
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(#[prop(optional)] message: &'static dyn PrintMessage) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let title_h1 = format!("Welcome to Leptos! {}", message.get_message());

    view! {
        <h1>{title_h1}</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}