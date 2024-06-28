use std::ops::Deref;

use leptos::*;

#[component]
pub fn ChildrenRefPage() -> impl IntoView {
    let div_ref = create_node_ref::<html::Div>();

    create_effect(move |_| {
        if let Some(div_elm) = document().get_element_by_id("myDiv") {
            let children = div_elm.children();
            let mut index = 0_u32;
            while let Some(elm) = children.item(index) {
                let inner_html = elm.inner_html();
                logging::debug_warn!("{inner_html}");
                index += 1;
            }
        }

        if let Some(div_elm) = div_ref.get() {
            let elm = div_elm.deref();
            let children = elm.children();
            let mut index = 0_u32;
            while let Some(elm) = children.item(index) {
                let inner_html = elm.inner_html();
                logging::debug_warn!("{inner_html}");
                index += 1;
            }
        }

        if let Some(div_elm) = div_ref.get() {
            let children = div_elm.children();
            let length = children.length();

            for index in 0..length {
                if let Some(elm) = children.item(index) {
                    let inner_html = elm.inner_html();
                    logging::debug_warn!("{inner_html}");
                }
            }

            let mut index = 0_u32;
            while let Some(elm) = children.item(index) {
                let inner_html = elm.inner_html();
                logging::debug_warn!("{inner_html}");
                index += 1;
            }
        }
    });

    view! {
        <div id="myDiv" node_ref=div_ref>
            <label>"label"</label>
            <input type="text"/>
        </div>
    }
}
