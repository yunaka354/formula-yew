use crate::components::DetailProps;
use crate::components::Spinner;
use crate::models::Lap;
use crate::utils;
use yew::prelude::*;

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
                let url = format!("http://localhost:3000/laps?year={}&round={}", year, round);
                let response = utils::fetch_server::<Vec<Lap>>(&url).await;
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
                            <Spinner />
                        }
                    }
                }
            }
        </div>
    }
}
