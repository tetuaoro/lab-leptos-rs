use std::time::Duration;

use leptos::{html::Dialog, *};

#[derive(Clone)]
pub struct Modal {
    title: String,
    message: String,
}

impl Modal {
    fn new() -> Self {
        Self {
            title: String::new(),
            message: String::new(),
        }
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_message(&self) -> String {
        self.message.clone()
    }

    fn is_empty(&self) -> bool {
        self.title.is_empty() && self.message.is_empty()
    }
}

impl From<(String, String)> for Modal {
    fn from((title, message): (String, String)) -> Self {
        Self { title, message }
    }
}

pub type ModalSignal = RwSignal<Modal>;

#[component]
pub fn ModalProvider(children: Children) -> impl IntoView {
    let modal_signal = create_rw_signal(Modal::new());
    provide_context(modal_signal);
    let modal_ctx = use_context::<ModalSignal>();
    let dialog_ref = create_node_ref::<Dialog>();

    let (modal, set_modal) = create_signal(Modal::new());

    create_effect(move |_| {
        if let Some(dialog_elm) = dialog_ref.get() {
            if let Some(modal) = modal_ctx.map(|rw_s| rw_s.get()) {
                if modal.is_empty() == false {
                    set_modal.set(Modal::from((modal.get_title(), modal.get_message())));

                    let _ = dialog_elm.show_modal();

                    set_timeout(
                        move || {
                            let _ = dialog_elm.close();
                        },
                        Duration::from_secs(7),
                    );
                }
            }
        }
    });

    let title = move || modal.get().get_title();
    let message = move || modal.get().get_message();

    view! {
        {children()}
        <dialog node_ref=dialog_ref>
            <h2>{title}</h2>
            <p>{message}</p>
        </dialog>
    }
}
