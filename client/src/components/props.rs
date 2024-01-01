use crate::models::StandingsBarChart;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct DetailProps {
    pub year: String,
    pub round: String,
}

#[derive(Properties, PartialEq, Clone)]
pub struct ChartProps {
    pub chart_data: StandingsBarChart,
}
