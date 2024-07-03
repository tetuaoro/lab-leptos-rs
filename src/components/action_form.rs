use leptos::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct SearchParams {
    search_for: String,
    whererumyluv: String,
}

#[server]
async fn search(data: SearchParams) -> Result<(), ServerFnError> {
    use std::{thread::sleep, time::Duration};
    sleep(Duration::from_secs(1));
    log::info!("params values {data:?}");
    Ok(())
}

#[component]
pub fn ActionSearchPage() -> impl IntoView {
    use leptos_router::ActionForm;

    let form_ref = create_node_ref::<html::Form>();
    let action_search = create_server_action::<Search>();
    let return_value = move || action_search.value().get();

    view! {
        <p>"Return value " {return_value}</p>
        <ActionForm node_ref=form_ref action=action_search>
            <label for="you">"Searching for"</label>
            <input id="you" attr:placeholder="a new girlfriend" name="data[search_for]" type="search" oninput="this.form.requestSubmit()"/>
            <input type="hidden" name="data[whererumyluv]" attr:value="notthereid10t"/>
        </ActionForm>
    }
}
