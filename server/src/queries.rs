use serde::Deserialize;

#[derive(Deserialize)]
pub struct RoundQuery {
    pub year: i32,
    pub round: i32,
}

#[derive(Deserialize)]
pub struct LapChartQuery {
    pub year: i32,
    pub round: i32,
    pub exclude_pitstop: bool,
}

#[derive(Deserialize)]
pub struct YearQuery {
    pub year: i32,
}
