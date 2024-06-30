use leptos::*;

#[derive(Debug, Clone)]
struct Data<'a> {
    id: u32,
    name: &'a str,
}

#[component]
pub fn TableFocusPage() -> impl IntoView {
    let table_ref = create_node_ref::<html::Table>();

    let on_click = move |_| {
        if let Some(table_elm) = table_ref.get() {
            _ = table_elm.focus();
        }
    };

    view! {
        <button on:click=on_click>"Focus table"</button>
        <Table node_ref=table_ref/>
    }
}

fn get_tabindex_elements(element: &web_sys::Element) -> Vec<web_sys::Element> {
    let children = element.children();
    let mut tabindexes = vec![];
    let mut index = 0;
    while let Some(child) = children.item(index) {
        if child.has_attribute("tabindex") {
            tabindexes.push(child.clone());
        }
        let mut tabindex_elements = get_tabindex_elements(&child);
        tabindexes.append(&mut tabindex_elements);
        index += 1;
    }
    tabindexes
}

#[derive(Debug, PartialEq)]
enum Sight {
    Next,
    Prev,
}

fn active_tabindex(elements: Vec<web_sys::Element>, sight: Sight) -> Result<(), ()> {
    let aria_active_selector = "aria-active";
    let mut element_active = None;
    for element in elements.iter() {
        if let Some(is_active) = element.get_attribute(aria_active_selector) {
            if is_active.eq("true") {
                element_active = Some(element);
                break;
            }
        }
    }

    use web_sys::wasm_bindgen::JsCast;

    // if no tab active
    if element_active.is_none() && elements.len() > 0 {
        if let Some(element) = elements.get(0) {
            _ = element.set_attribute(aria_active_selector, "true");
            if let Some(tr_elm) = element.dyn_ref::<web_sys::HtmlElement>() {
                _ = tr_elm.focus();
            }
        }
    }

    // if some tab active
    if let Some(element) = element_active {
        let mut selected_element = None;
        if sight == Sight::Prev {
            if let Some(prev_element) = element.previous_element_sibling() {
                selected_element = Some(prev_element);
            }
        }

        if sight == Sight::Next {
            if let Some(next_element) = element.next_element_sibling() {
                selected_element = Some(next_element);
            }
        }

        if let Some(selected_element) = selected_element {
            _ = element.set_attribute(aria_active_selector, "false");
            _ = selected_element.set_attribute(aria_active_selector, "true");
            if let Some(tr_elm) = selected_element.dyn_ref::<web_sys::HtmlElement>() {
                _ = tr_elm.focus();
            }
        }
    }

    Err(())
}

#[component]
fn Table(node_ref: NodeRef<html::Table>) -> impl IntoView {
    let table_data = vec![
        Data {
            id: 1,
            name: "Toto",
        },
        Data {
            id: 2,
            name: "Tata",
        },
    ];

    let signal = create_rw_signal(vec![]);
    create_effect(move |prev| {
        if let Some(is_set) = prev {
            if is_set {
                return true;
            }
        }

        if let Some(table_elm) = node_ref.get() {
            use web_sys::wasm_bindgen::JsCast;
            if let Some(element) = table_elm.dyn_ref::<web_sys::Element>() {
                let tabindexes = get_tabindex_elements(element);
                signal.set(tabindexes);
            }
        }

        true
    });

    let on_kd = move |evt: ev::KeyboardEvent| {
        evt.prevent_default();
        let key = evt.key();
        if key.eq("ArrowUp") {
            _ = active_tabindex(signal.get(), Sight::Prev);
        }
        if key.eq("ArrowDown") {
            _ = active_tabindex(signal.get(), Sight::Next);
        }
    };

    view! {
        <table node_ref=node_ref on:keydown=on_kd attr:tabindex="-1">
            <thead>
                <tr>
                    <th>"ID"</th>
                    <th>"Name"</th>
                </tr>
            </thead>
            <tbody>
                <For each=move || table_data.clone() key=|k| k.id let:data>
                    <tr attr:tabindex="-1" attr:aria-active="false">
                        <td>{data.id}</td>
                        <td>{data.name}</td>
                    </tr>
                </For>
            </tbody>
        </table>
    }
}
