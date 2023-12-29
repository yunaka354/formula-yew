use gloo_net::http::Request;
use yew::prelude::*;

mod models;
use models::Race;

#[function_component(App)]
fn app() -> Html {
    let races = use_state(|| None);
    {
        let races = races.clone();
        use_effect_with((), move |_| {
            let races = races.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = "http://localhost:3000/races?year=2023";
                let response: Vec<Race> = Request::get(url)
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
                                                        <td>{ &race.race_name }</td>
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

fn main() {
    yew::Renderer::<App>::new().render();
}
