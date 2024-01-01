use yew::prelude::*;

use crate::{models::Season, utils};

#[function_component(Seasons)]
pub fn seasons() -> Html {
    let seasons = use_state(|| None);
    {
        let seasons = seasons.clone();
        use_effect_with((), move |_| {
            let seasons = seasons.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = "http://localhost:3000/seasons";
                let response = utils::fetch_server::<Vec<Season>>(url).await;
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
                                                <li>
                                                    <a
                                                        href={ format!("./races?year={}", season.season.clone()) }
                                                        class="text-blue-500 hover:text-blue-700"
                                                    >
                                                        { season.season.clone() }
                                                    </a>
                                                </li>
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
