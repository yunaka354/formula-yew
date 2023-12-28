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
            laps: None,
        }
    ];

    html! {
        <div>
            {
                for races.iter().map(|race| {
                    html! {
                        <div>
                            <p>{ format!("Season: {}", race.season) }</p>
                            <p>{ format!("Round: {}", race.round) }</p>
                            <p>{ format!("Circuit Name: {}", race.circuit_name) }</p>
                        </div>
                    }
                })
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}