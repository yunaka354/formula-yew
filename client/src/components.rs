use crate::models::{Race, RaceResult, Season};
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

#[function_component(Results)]
pub fn results() -> Html {
    let location = use_location().unwrap();
    // NOTE: location.query_str() returns a string with a leading "?"
    let query_string = location.query_str().replace("?", "");
    let query_params: HashMap<String, String> =
        serde_urlencoded::from_str(&query_string).unwrap_or_default();

    let year = query_params.get("year").cloned().unwrap_or_default();
    let round = query_params.get("round").cloned().unwrap_or_default();
    let title = format!("Formula 1 {} Round {}", &year, &round);

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
                                <h1>{ title }</h1>
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
