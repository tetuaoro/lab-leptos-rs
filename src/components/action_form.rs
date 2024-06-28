use leptos::*;

#[server]
async fn search(search_for: String) -> Result<String, ServerFnError> {
    use std::{thread::sleep, time::Duration};
    sleep(Duration::from_secs(1));
    Ok(search_for)
}

#[component]
pub fn ActionSearchPage() -> impl IntoView {
    use leptos_router::ActionForm;

    let action_search = create_server_action::<Search>();

    let on_input = move |evt: ev::Event| {
        evt.prevent_default();
        let value = event_target_value(&evt);
        action_search.dispatch(Search { search_for: value });
        logging::log!("Hello Babe !");
    };

    let return_value = move || action_search.value().get();

    view! {
        <p>"Return value " {return_value}</p>
        <ActionForm action=action_search>
            <label for="you">"Searching for"</label>
            <input id="you" attr:placeholder="a new girlfriend" name="search_for" on:input=on_input/>
        </ActionForm>
    }
}
