pub mod counter;
pub mod home;
pub mod moffu;
pub mod modal;

use counter::CounterPage;
use home::HomePage;
use moffu::MoffuPage;
use modal::ModalPage;

use leptos::*;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <HomePage/>
        <CounterPage/>
        <MoffuPage/>
        <ModalPage/>

        <div id="app" style="margin:20px">
            "App"
        </div>
        <div id="app1">"App1"</div>
        <div id="app2">"App2"</div>
    }
}
