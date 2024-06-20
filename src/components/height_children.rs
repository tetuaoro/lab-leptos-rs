use leptos::*;

#[component]
fn HeightWrapper(children: Children) -> impl IntoView {
    let div_ref = create_node_ref::<html::Div>();

    create_effect(move |run_once| {
        if let Some(ro) = run_once {
            if ro {
                return true;
            }
        }

        // run once when false
        logging::log!("logging create_effect");

        if let Some(elm) = div_ref.get() {
            let height = elm.client_height();
            logging::log!("height {height}");
        }

        true
    });

    view! {
        <div node_ref=div_ref>{children()}</div>
    }
}

#[component]
fn HeightChild() -> impl IntoView {
    let style = r"
        display: block;
        background-color: red;
        width: 100px;
        height: 100px;
    ";

    view! { <div style=style></div> }
}

#[component]
pub fn HeightParent() -> impl IntoView {
    view! {
        <HeightWrapper>
            <HeightChild/>
        </HeightWrapper>
    }
}
