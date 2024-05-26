use serde::{Deserialize, Serialize};

use crate::adapter::UserAdapter;

#[derive(Default, Clone, Serialize, Deserialize)]
enum ROLE {
    #[default]
    LAMBDA,
    ADMIN,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct User {
    username: String,
    fullname: String,
    role: ROLE,
}

impl UserAdapter for User {
    fn get_username(&self) -> String {
        self.username.clone()
    }
}
