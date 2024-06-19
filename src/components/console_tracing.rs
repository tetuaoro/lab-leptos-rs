use leptos::*;

#[component]
pub fn ConsoleLogPage() -> impl IntoView {
    let initial_title = String::from("Hello, world.");
    let (title, set_title) = create_signal(initial_title);

    log::info!("log info HomePage component");

    view! {
        <form method="GET" action="">
            <label>
                "I want to read:"
                <input
                    type="text"
                    prop:value=title
                    on:input=move |evt| {
                        let input = event_target_value(&evt);
                        log::info!("log info Input: {input}");
                        logging::log!("logging log Input: {input}");
                        set_title.set(input);
                    }
                />

            </label>
            <button>"Create"</button>
        </form>
    }
}
