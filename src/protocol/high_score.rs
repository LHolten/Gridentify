use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HighScore {
    pub name: String,
    pub score: u32,
}
