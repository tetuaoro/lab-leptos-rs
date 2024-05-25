use leptos::*;

/// Renders the moffu page of your application.
#[component]
pub fn MoffuPage() -> impl IntoView {
    view! {
        <Something/>

        <div style="margin:20px"></div>

        <ProvidePortalContext>
            <PortalTrigger/>
            <PortalContent app="app1">
                <ClosePortal/>
            </PortalContent>
            <PortalTrigger/>
            <PortalContent app="app2">
                <ClosePortal/>
            </PortalContent>
        </ProvidePortalContext>

        <PortalArray/>
    }
}

#[derive(Clone)]
pub struct ContextPortal(RwSignal<bool>);

#[component]
pub fn ProvidePortalContext(children: Children) -> impl IntoView {
    let signal = create_rw_signal(false);
    // create_effect(move |_| log::info!("change on provide portal {}: {}", name, signal.get()));

    provide_context(ContextPortal(signal));
    children()
}

#[component]
pub fn PortalTrigger() -> impl IntoView {
    let signal = use_context::<ContextPortal>().expect("signal").0;

    view! { <button on:click=move |_| signal.update(|value| *value = true)>"Portal Trigger"</button> }
}

#[component]
pub fn ClosePortal() -> impl IntoView {
    let signal = use_context::<ContextPortal>().expect("signal").0;
    view! { <button on:click=move |_| signal.update(|value| *value = false)>"Close portal"</button> }
}

#[component]
pub fn PortalContent(children: ChildrenFn, app: &'static str) -> impl IntoView {
    let signal = use_context::<ContextPortal>().expect("signal").0;

    view! {
        <Show when=move || signal.get()>
            <Portal mount=document().get_element_by_id(app).unwrap() clone:children>
                <div>{children.clone()}</div>
            </Portal>
        </Show>
    }
}

#[component]
fn Something() -> impl IntoView {
    let signal_one = create_rw_signal(false);
    let signal_two = create_rw_signal(false);
    view! {
        <div>
            <button on:click=move |_| signal_one.set(true)>"Button one"</button>
            <Show when=move || signal_one.get()>
                <Portal mount=document().get_element_by_id("app").unwrap()>
                    <button on:click=move |_| signal_one.set(false)>"Close me button one"</button>
                </Portal>
            </Show>

            <button on:click=move |_| signal_two.set(true)>"Button two"</button>
            <Show when=move || signal_two.get()>
                <Portal mount=document().get_element_by_id("app").unwrap()>
                    <button on:click=move |_| signal_two.set(false)>"Close me button two"</button>
                </Portal>
            </Show>
        </div>
    }
}

#[component]
fn PortalArray() -> impl IntoView {
    let signals = create_rw_signal([false; 3]);

    let update_signal = move |index: usize, value: bool| {
        let mut new_value = signals.get();
        for (i, _) in new_value.iter().enumerate() {
            if i == index {
                new_value[i] = value;
                break;
            }
        }
        signals.set(new_value);
    };

    view! {
        <div>
            <For
                each=move || signals.get().into_iter().enumerate()
                key=|(index, value)| (index.clone(), value.clone())
                let:signal
            >
                <button on:click=move |_| update_signal(
                    signal.0,
                    true,
                )>{format!("Button open {}", signal.0)}</button>
                <Show when=move || signal.1>
                    <Portal mount=document().get_element_by_id("app").unwrap()>
                        <button on:click=move |_| update_signal(
                            signal.0,
                            false,
                        )>{format!("Button close {}", signal.0)}</button>
                    </Portal>
                </Show>

            </For>

        </div>
    }
}
