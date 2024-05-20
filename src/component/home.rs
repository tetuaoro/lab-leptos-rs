use crate::adapter::PrintMessage;
use crate::entity::Message;
use leptos::*;
use std::time::Duration;

static NAME: &str = "Tetuaoro";

/// Renders the home page of your application.
#[component]
pub fn HomeWrapper() -> impl IntoView {
    let message = Message::from(String::from(NAME));
    let (rd, wd) = create_signal(message);

    create_effect(move |_| {
        set_interval(
            move || {
                wd.update(|d| {
                    let mes = d.get_message();
                    if mes.eq(&String::from(NAME)) {
                        d.update("Hereiti");
                    } else {
                        d.update(NAME);
                    }
                })
            },
            Duration::from_secs(3),
        );
    });

    view! {
        <HomePage data=rd />
    }
}

#[component]
fn HomePage<P: PrintMessage + 'static>(data: ReadSignal<P>) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let title_h1 = move || data.with(|d| d.get_message());

    view! {
        <h1>"Welcome " {title_h1}</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
