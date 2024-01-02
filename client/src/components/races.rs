use crate::components::table::{Table, TableContents};
use crate::components::Spinner;
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
                        let contents = TableContents::convert_races(races);
                        html! {
                            <>
                                <h1>{ title }</h1>
                                <Table contents={contents} />
                            </>
                        }
                    },
                    None => {
                        html! {
                            <Spinner />
                        }
                    }
                }
            }
        </div>
    }
}
