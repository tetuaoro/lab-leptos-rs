use leptos::*;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
struct SearchParams {
    search_for: String,
    whererumyluv: String,
}

#[server]
async fn search(params: SearchParams) -> Result<(), ServerFnError> {
    use std::{thread::sleep, time::Duration};
    sleep(Duration::from_secs(1));
    log::info!("params values {params:?}");
    Ok(())
}

#[component]
pub fn ActionSearchPage() -> impl IntoView {
    use leptos_router::ActionForm;

    let form_ref = create_node_ref::<html::Form>();

    let action_search = create_server_action::<Search>();

    let on_input = move |evt: ev::Event| {
        if let Some(form_elm) = form_ref.get() {
            if let Ok(form_data) = web_sys::FormData::new_with_form(&form_elm) {
                evt.prevent_default();
                let search_for = form_data.get("search_for").as_string().unwrap();
                let whererumyluv = form_data.get("whererumyluv").as_string().unwrap();
                let params = SearchParams {
                    search_for,
                    whererumyluv,
                };
                action_search.dispatch(Search { params });
            }
        }
    };

    let return_value = move || action_search.value().get();

    view! {
        <p>"Return value " {return_value}</p>
        <ActionForm node_ref=form_ref action=action_search>
            <label for="you">"Searching for"</label>
            <input id="you" attr:placeholder="a new girlfriend" name="search_for" on:input=on_input/>
            <input name="whererumyluv" attr:value="notthereid10t"/>
        </ActionForm>
    }
}
