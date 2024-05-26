mod counter;
mod home;
mod moffu;
mod modal;
mod user;

use counter::CounterPage;
use home::HomePage;
use moffu::MoffuPage;
use modal::ModalPage;
use user::UserPage;

use leptos::*;

#[component]
pub fn Page() -> impl IntoView {
    view! {
        <UserPage/>
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
