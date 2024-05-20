use leptos::*;

#[cfg(feature = "ssr")]
use rand::Rng;

#[server(Counter)]
pub async fn counter() -> Result<i32, ServerFnError> {
    let mut rng = rand::thread_rng();
    Ok(rng.gen())
}
