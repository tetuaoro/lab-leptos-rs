use leptos::*;

#[component]
pub fn ConsoleLogPage() -> impl IntoView {
    let initial_title = String::from("Hello, world.");
    let (title, set_title) = create_signal(initial_title);

    leptos::logging::log!("HomePage component");

    view! {
        <form method="GET" action="">
            <label>
                "I want to read:"
                <input
                    type="text"
                    prop:value=title
                    on:input=move |evt| {
                        let input = event_target_value(&evt);
                        leptos::logging::log!("logging Input: {input}");
                        set_title.set(input);
                    }
                />

            </label>
            <button>"Create"</button>
        </form>
    }
}
