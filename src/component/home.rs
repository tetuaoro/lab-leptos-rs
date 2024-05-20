use crate::adapter::PrintMessage;
use crate::entity::Message;
use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomeWrapper() -> impl IntoView {
    let message = Message::from(String::from("tetuaoro"));
    let wm = Box::new(message);

    view! {
        <HomePage data=wm />
    }
}

#[component]
fn HomePage<P: PrintMessage>(data: Box<P>) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let title_h1 = format!("Welcome to Leptos! {}", data.get_message());

    view! {
        <h1>{title_h1}</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
