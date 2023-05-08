use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Plant {
    pub name: String,
    pub species: String,
}
