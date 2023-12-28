use serde::Deserialize;

#[derive(Deserialize)]
pub struct Round {
    pub year: i32,
    pub round: i32,
}
