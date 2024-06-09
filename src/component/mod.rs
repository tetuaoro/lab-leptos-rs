mod counter;
mod home;
mod modal;
mod moffu;
pub mod slug;
mod user;
mod grid_template;

// use counter::CounterPage;
// use home::HomePage;
// use modal::ModalPage;
// use moffu::MoffuPage;
pub use grid_template::*;

use leptos::*;

#[component]
pub fn Page() -> impl IntoView {
    view! { <GridPage/> }
}
