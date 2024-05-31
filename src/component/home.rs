use crate::adapter::PrintMessage;
use crate::entity::Message;

use leptos::*;
use leptos_router::A;
use std::time::Duration;

static TETUAORO: &str = "Tetuaoro";
static HEREITI: &str = "Hereiti";

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let message = Message::from(String::from(TETUAORO));
    let (rd, wd) = create_signal(message);

    create_effect(move |_| {
        set_interval(
            move || {
                wd.update(|d| {
                    let mes = d.get_message();
                    if mes.eq(&String::from(TETUAORO)) {
                        d.update(HEREITI);
                    } else {
                        d.update(TETUAORO);
                    }
                })
            },
            Duration::from_secs(3),
        );
    });

    view! { <Home data=rd/> }
}

#[component]
fn Home<P: PrintMessage + 'static>(data: ReadSignal<P>) -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let title_h1 = move || data.with(|d| d.get_message());

    view! {
        <h1>"Welcome " {title_h1}</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <A href="clock">"Go to Clock page"</A>
    }
}
