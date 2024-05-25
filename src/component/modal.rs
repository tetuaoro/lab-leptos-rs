use leptos::*;

use crate::{alert::{Modal, ModalSignal}, api::numera};

/// Renders the modal page of your application.
#[component]
pub fn ModalPage() -> impl IntoView {
    let modal_ctx = use_context::<ModalSignal>();

    let on_click = move |_| {
        spawn_local(async move {
            if let Some(signal) = modal_ctx {
                if let Ok(numera) = numera().await {
                    signal.set(Modal::from((
                        String::from("'Ahuru ma"),
                        numera,
                    )))
                    
                }
            }
        });
    };

    view! { <button on:click=on_click>"Modal btn"</button> }
}
