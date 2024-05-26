use crate::entity::User;
use cfg_if::cfg_if;
use leptos::*;

cfg_if! { if #[cfg(feature = "ssr")] {
    use once_cell::sync::Lazy;
    use surrealdb::{
        engine::remote::ws::{Client, Wss},
        Surreal
    };
    use rand::Rng;
    use rand::seq::IteratorRandom;

    const NUMERA: [&str; 4] = ["hō'ē", "piti", "toru", "maha"];
    const DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

    async fn get_sdb() -> Result<Surreal<Client>, ServerFnError> {
        if let Ok(_) = DB.version().await {
            return Ok(DB.to_owned());
        }

        DB.connect::<Wss>("address").await?;
        DB.use_ns("ns").use_db("db").await?;
        Ok(DB.to_owned())
    }

    static USER_TABLE: &'static str = "user";
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

#[server(GetUsers)]
pub async fn get_users() -> Result<Vec<User>, leptos::ServerFnError> {
    let sdb = get_sdb().await?;
    let persons = sdb.select::<Vec<User>>(USER_TABLE).await?;
    Ok(persons)
}
