use yew::prelude::*;

mod models;
use models::Race;

#[function_component(App)]
fn app() -> Html {
    let races = vec![
        Race {
            season: 2021,
            round: 5,
            circuit_name: "Circuit de Monaco".to_string(),
            date: "May 23, 2021".to_string(),
            laps: None,
        },
        Race {
            season: 2021,
            round: 14,
            circuit_name: "Autodromo Nazionale di Monza".to_string(),
            date: "September 12, 2021".to_string(),
            laps: None,
        },
        Race {
            season: 2021,
            round: 15,
            circuit_name: "Sochi Autodrom".to_string(),
            date: "September 26, 2021".to_string(),
            laps: Some(50),
        }
    ];

    html! {
        // table header for race entities
        <div>
            <h1>{ "Formula 1 2021 Season" }</h1>
            <table>
                <thead>
                    <tr>
                        <th>{ "Season" }</th>
                        <th>{ "Round" }</th>
                        <th>{ "Circuit" }</th>
                        <th>{ "Date" }</th>
                        <th>{ "Laps" }</th>
                    </tr>
                </thead>
                <tbody>
                    {
                        for races.iter().map(|race| {
                            // map out attributes in race entity
                            html! {
                                <tr>
                                    <td>{ race.season }</td>
                                    <td>{ race.round }</td>
                                    <td>{ &race.circuit_name }</td>
                                    <td>{ &race.date }</td>
                                    <td>{ race.laps }</td>
                                </tr>
                            }
                        })
                    }
                </tbody>
            </table>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}