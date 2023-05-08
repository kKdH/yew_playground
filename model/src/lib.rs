use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Plant {
    pub name: String,
    pub species: String,
}
