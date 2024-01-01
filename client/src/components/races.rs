use crate::models::Race;
use crate::utils;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Races)]
pub fn races() -> Html {
    let races = use_state(|| None);
    let location = use_location().unwrap();
    // NOTE: location.query_str() returns a string with a leading "?"
    let query_string = location.query_str().replace("?", "");
    let query_params: HashMap<String, String> =
        serde_urlencoded::from_str(&query_string).unwrap_or_default();

    let year = query_params.get("year").cloned().unwrap_or_default();
    let title = format!("Formula 1 {} Season", year);
    {
        let races = races.clone();
        use_effect_with((), move |_| {
            let races = races.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("http://localhost:3000/races?year={}", year);
                let response = utils::fetch_server::<Vec<Race>>(&url).await;
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
                                <h1>{ title }</h1>
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
                                                        <td>
                                                            <a
                                                                href={format!("/results?year={}&round={}", race.season, race.round)}
                                                                class="text-blue-500 hover:text-blue-700"
                                                            >
                                                                { &race.race_name }
                                                            </a>
                                                        </td>
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
