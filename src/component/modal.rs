use leptos::*;

use crate::alert::{Modal, ModalSignal};

/// Renders the modal page of your application.
#[component]
pub fn ModalPage() -> impl IntoView {
    let modal_ctx = use_context::<ModalSignal>();

    let on_click = move |_| {
        if let Some(signal) = modal_ctx {
            signal.set(Modal::from((String::from("ahuru ma"), String::from("hō'e"))))
        }
    };
    
    view! { <button on:click=on_click>"Modal btn"</button> }
}
