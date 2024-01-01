use crate::components::DetailProps;
use crate::components::Spinner;
use crate::models::RaceResult;
use crate::utils;
use yew::prelude::*;

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
                let response = utils::fetch_server::<Vec<RaceResult>>(&url).await;
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
                            <Spinner />
                        }
                    }
                }
            }
        </div>
    }
}
