mod counter;
mod home;
mod modal;
mod moffu;
pub mod slug;
mod user;

use counter::CounterPage;
use home::HomePage;
use modal::ModalPage;
use moffu::MoffuPage;
// use user::UserPage;

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
