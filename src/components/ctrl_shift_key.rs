use leptos::{ev::KeyboardEvent, *};

#[component]
pub fn CtrlShiftKey() -> impl IntoView {

    let on_kevent = move |evt: KeyboardEvent| {
        evt.prevent_default();
        let c = evt.ctrl_key();
        let s = evt.shift_key();
        logging::log!("logging log c {c}, s {s}");
    };

    view! {
        <label for="babies">"Type & check ctrl & shift key :"</label>
        <input id="babies" on:keydown=on_kevent/>
    }
}
