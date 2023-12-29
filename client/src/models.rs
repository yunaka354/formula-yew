use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Race {
    pub season: u32,
    pub round: u32,
    pub race_name: String,
    pub circuit_name: String,
    pub date: String,
}
