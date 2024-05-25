pub mod counter;
pub mod home;
pub mod moffu;

use counter::CounterPage;
use home::HomePage;
use moffu::MoffuPage;

use leptos::*;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <HomePage/>
        <CounterPage/>
        <MoffuPage/>

        <div id="app" style="margin:20px">
            "App"
        </div>
        <div id="app1">"App1"</div>
        <div id="app2">"App2"</div>
    }
}
