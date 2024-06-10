use html::Input;
use leptos::{ev::*, *};
use std::ops::Add;

#[derive(Debug, Default, Clone)]
struct SliceSignalStruct {
    names: Vec<String>,
    how_many_in_your_life: u8, // u0 for me xD
}

impl SliceSignalStruct {
    fn with(surname: String) -> Self {
        Self {
            names: vec![surname],
            how_many_in_your_life: Default::default(),
        }
    }
}

#[component]
pub fn SliceSignalPage() -> impl IntoView {
    let input_ref = create_node_ref::<Input>();
    let the_first_love = SliceSignalStruct::with("Babe".into());
    let signal = create_rw_signal(the_first_love);

    let getter = |signal: &SliceSignalStruct| signal.names.clone();
    let setter = move |state: &mut SliceSignalStruct, new_value| {
        state.names.push(new_value);
        let _ = state.how_many_in_your_life.add(1);

        if let Some(elm) = input_ref.get() {
            let _ = elm.focus();
        }
    };
    let (surnames, set_surnames) = create_slice(signal, getter, setter);

    let on_input = move |evt: Event| {
        let value = event_target_value(&evt);
        set_surnames.set(value);
    };

    view! {
        <label for="babies">"What's her surname :"</label>
        <input node_ref=input_ref id="babies" on:input=on_input/>
        <ul>
            <For each=move || surnames.get() key=|surname| surname.clone() let:surname>
                <li>{surname}</li>
            </For>
        </ul>
    }
}
