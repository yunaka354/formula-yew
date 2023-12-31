use crate::models::{Race, RaceResult, Season, Lap};
use gloo_net::http::Request;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Seasons)]
pub fn seasons() -> Html {
    let seasons = use_state(|| None);
    {
        let seasons = seasons.clone();
        use_effect_with((), move |_| {
            let seasons = seasons.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = "http://localhost:3000/seasons";
                let response: Vec<Season> = Request::get(url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                seasons.set(Some(response));
            });
            || ()
        });
    }

    html! {
        <div>
            {
                match (*seasons).clone() {
                    Some(seasons) => {
                        html! {
                            <>
                                <h1>{ "Formula 1 Seasons" }</h1>
                                <ul>
                                    {
                                        for seasons.iter().map(|season| {
                                            html! {
                                                <li><a href={ format!("./races?year={}", season.season.clone()) }>{ season.season.clone() }</a></li>
                                            }
                                        })
                                    }
                                </ul>
                            </>
                        }
                    },
                    None => {
                        html! {
                            <h1>{ "Loading..." }</h1>
                        }
                    }
                }
            }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct DetailProps {
    year: String,
    round: String,
}

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
            <Results year={year.clone()} round={round.clone()} />
            <Laps year={year} round={round} />
        </div>
    }
}

#[function_component(Laps)]
pub fn laps(props: &DetailProps) -> Html {
    let year = props.year.clone();
    let round = props.round.clone();
    let laps = use_state(|| None);
    {
        let laps = laps.clone();
        use_effect_with((), move |_| {
            let laps = laps.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!(
                    "http://localhost:3000/laps?year={}&round={}",
                    year, round
                );
                let response: Vec<Lap> = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                laps.set(Some(response));
            });
            || ()
        });
    }

    html! {
        <div>
            {
                match (*laps).clone() {
                    Some(laps) => {
                        html! {
                            <>
                                <table>
                                    <thead>
                                        <tr>
                                            <th>{ "Driver ID" }</th>
                                            <th>{ "Lap" }</th>
                                            <th>{ "Position" }</th>
                                            <th>{ "Lap Time" }</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {
                                            for laps.iter().map(|lap| {
                                                html! {
                                                    <tr>
                                                        <td>{ lap.driver_id.clone() }</td>
                                                        <td>{ lap.lap }</td>
                                                        <td>{ lap.position }</td>
                                                        <td>{ lap.time.clone() }</td>
                                                    </tr>
                                                }
                                            })
                                        }
                                    </tbody>
                                </table>
                            </>
                        }
                    },
                    None => {
                        html! {
                            <h1>{ "Loading..." }</h1>
                        }
                    }
                }
            }
        </div>
    }
}

#[function_component(Results)]
pub fn results(props: &DetailProps) -> Html {
    let year = props.year.clone();
    let round = props.round.clone();
    let results = use_state(|| None);
    {
        let results = results.clone();
        use_effect_with((), move |_| {
            let results = results.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!(
                    "http://localhost:3000/results?year={}&round={}",
                    year, round
                );
                let response: Vec<RaceResult> = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                results.set(Some(response));
            });
            || ()
        });
    }

    html! {
        <div>
            {
                match (*results).clone() {
                    Some(results) => {
                        html! {
                            <>
                                <table>
                                    <thead>
                                        <tr>
                                            <th>{ "Position" }</th>
                                            <th>{ "Code" }</th>
                                            <th>{ "Driver" }</th>
                                            <th>{ "Team" }</th>
                                            <th>{ "Points" }</th>
                                            <th>{ "Status" }</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {
                                            for results.iter().map(|result| {
                                                html! {
                                                    <tr>
                                                        <td>{ result.position }</td>
                                                        <td>{ result.code.clone() }</td>
                                                        <td>{ format!("{} {}", result.given_name.clone(), result.family_name.clone()) }</td>
                                                        <td>{ result.constructor.clone() }</td>
                                                        <td>{ result.points }</td>
                                                        <td>{ result.status.clone() }</td>
                                                    </tr>
                                                }
                                            })
                                        }
                                    </tbody>
                                </table>
                            </>
                        }
                    },
                    None => {
                        html! {
                            <h1>{ "Loading..." }</h1>
                        }
                    }
                }
            }
        </div>
    }
}

#[function_component(Races)]
pub fn races() -> Html {
    let races = use_state(|| None);
    let location = use_location().unwrap();
    // NOTE: location.query_str() returns a string with a leading "?"
    let query_string = location.query_str().replace("?", "");
    let query_params: HashMap<String, String> =
        serde_urlencoded::from_str(&query_string).unwrap_or_default();

    let year = query_params.get("year").cloned().unwrap_or_default();
    {
        let races = races.clone();
        use_effect_with((), move |_| {
            let races = races.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://localhost:3000/races?year={}", year);
                let response: Vec<Race> = Request::get(&url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                races.set(Some(response));
            });
            || ()
        });
    }

    html! {
        <div>
            {
                match (*races).clone() {
                    Some(races) => {
                        html! {
                            <>
                                <h1>{ "Formula 1 2023 Season" }</h1>
                                <table>
                                    <thead>
                                        <tr>
                                            <th>{ "Season" }</th>
                                            <th>{ "Round" }</th>
                                            <th>{ "Title" }</th>
                                            <th>{ "Circuit" }</th>
                                            <th>{ "Date" }</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {
                                            for races.iter().map(|race| {
                                                html! {
                                                    <tr>
                                                        <td>{ race.season }</td>
                                                        <td>{ race.round }</td>
                                                        <td><a href={format!("/results?year={}&round={}", 2023, race.round)}> { &race.race_name } </a></td>
                                                        <td>{ &race.circuit_name }</td>
                                                        <td>{ &race.date }</td>
                                                    </tr>
                                                }
                                            })
                                        }
                                    </tbody>
                                </table>
                            </>
                        }
                    },
                    None => {
                        html! {
                            <h1>{ "Loading..." }</h1>
                        }
                    }
                }
            }
        </div>
    }
}
