use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Attack {
    pub name: String,
    pub input: String,
    pub damage: String,
    pub block: String,
    pub cancel: String,
    pub property: String,
    pub cost: String,
    pub attribute: String,
    pub startup: String,
    pub active: String,
    pub recovery: String,
    pub overall: String,
    pub advantage: String,
    pub invuln: String,
}
