use dict_derive::IntoPyObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, IntoPyObject)]
pub struct HighScore {
    pub(crate) name: String,
    pub(crate) score: u32,
}
