use crate::components::{laps::LapsBox, Laps, Results, Standings};
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
        <>
            <div class={"text-center py-4"}> // Center the title and add vertical padding
                <h1 class={"text-2xl font-bold mb-4"}>{ title }</h1> // Add bottom margin to the title
            </div>
            <Standings year={year.clone()} round={round.clone()} />
            <Laps year={year.clone()} round={round.clone()} />
            <LapsBox year={year.clone()} round={round.clone()} />
            <Results year={year.clone()} round={round.clone()} />
        </>
    }
}
