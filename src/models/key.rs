use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Key {
    pub key: String,
    pub username: String,
    pub added_at: String,
}