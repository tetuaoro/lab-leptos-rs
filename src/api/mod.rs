use cfg_if::cfg_if;
use leptos::*;

cfg_if! { if #[cfg(feature = "ssr")] {
    use rand::Rng;
    use rand::seq::IteratorRandom;
    const NUMERA: [&str; 4] = ["hō'ē", "piti", "toru", "maha"];
}}

#[server(Counter)]
pub async fn counter() -> Result<i32, ServerFnError> {
    let mut rng = rand::thread_rng();
    Ok(rng.gen())
}

#[server(Numera)]
pub async fn numera() -> Result<String, ServerFnError> {
    let mut rng = rand::thread_rng();
    let result = NUMERA
        .into_iter()
        .choose(&mut rng)
        .map_or(Default::default(), |value| String::from(value));
    Ok(result)
}
