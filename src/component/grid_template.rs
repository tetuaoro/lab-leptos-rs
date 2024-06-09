use ev::MouseEvent;
use leptos::*;

use crate::api::{counter, numera};

#[component]
pub fn GridPage() -> impl IntoView {
    let (row, set_row) = create_signal(vec![]);

    let on_click = move |_evt: MouseEvent| {
        spawn_local(async move {
            if let Ok(count) = counter().await {
                if let Ok(msg) = numera().await {
                    set_row.update(|r| r.push(format!("{} {}", msg, count)));
                }
            }
        });
    };

    let style = r"'head head'
    'nav main'
    'nav foot'";

    view! {
        <section id="page" style:grid-template-areas=style>
            <div id="header">"En-tête"</div>
            <div id="nav">"Navigation"</div>
            <div id="main">"Zone principale"</div>
            <div id="footer">"Pied de page"</div>
            <For each=move || row.get() key=|k| k.clone() let:msg>
                <div id="main-baby">{msg}</div>
            </For>
        </section>
        <button on:click=on_click>"Push me babe !"</button>
    }
}
