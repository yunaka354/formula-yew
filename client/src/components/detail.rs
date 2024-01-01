use crate::components::{Laps, Results, Standings};
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Detail)]
pub fn detail() -> Html {
    let location = use_location().unwrap();
    // NOTE: location.query_str() returns a string with a leading "?"
    let query_string = location.query_str().replace("?", "");
    let query_params: HashMap<String, String> =
        serde_urlencoded::from_str(&query_string).unwrap_or_default();

    let year = query_params.get("year").cloned().unwrap_or_default();
    let round = query_params.get("round").cloned().unwrap_or_default();
    let title = format!("Formula 1 {} Round {}", year, round);

    html! {
        <div>
            <h1>{ title }</h1>
            <Standings year={year.clone()} round={round.clone()} />
            <Results year={year.clone()} round={round.clone()} />
            <Laps year={year.clone()} round={round.clone()} />
        </div>
    }
}
