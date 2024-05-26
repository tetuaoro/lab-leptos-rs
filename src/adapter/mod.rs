use serde::{Deserialize, Serialize};

pub trait PrintMessage: Default {
    fn get_message(&self) -> String;
}

pub trait UserAdapter: Clone + Serialize + for<'b> Deserialize<'b> {
    fn get_username(&self) -> String;
}
