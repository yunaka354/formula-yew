use crate::components::DetailProps;
use crate::components::LapChart;
use crate::components::Spinner;
use crate::components::charts::LapBoxChart;
use crate::models::LapLineChartData;
use crate::utils;
use yew::prelude::*;

#[function_component(Laps)]
pub fn laps(props: &DetailProps) -> Html {
    let props = props.clone();
    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect_with((), move |_| {
            let data = data.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let data = data.clone();
                let url = format!(
                    "http://localhost:3000/laps-chart?year={}&round={}",
                    props.year, props.round
                );
                let response = utils::fetch_server::<Vec<LapLineChartData>>(&url).await;
                data.set(Some(response));
            });
        });
    }

    html! {
        <>
            {
                match (*data).clone() {
                    None => {
                        html! {
                            <Spinner />
                        }
                    },
                    Some(data) => {
                        html! {
                            <LapChart chart_data={data} plot_id={"plot-lap"}/>
                        }
                    }
                }
            }
        </>
    }
}

#[function_component(LapsBox)]
pub fn laps_box(props: &DetailProps) -> Html {
    let props = props.clone();
    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect_with((), move |_| {
            let data = data.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let data = data.clone();
                let url = format!(
                    "http://localhost:3000/laps-chart?year={}&round={}",
                    props.year, props.round
                );
                let response = utils::fetch_server::<Vec<LapLineChartData>>(&url).await;
                data.set(Some(response));
            });
        });
    }

    html! {
        <>
            {
                match (*data).clone() {
                    None => {
                        html! {
                            <Spinner />
                        }
                    },
                    Some(_data) => {
                        html! {
                            <LapBoxChart />
                        }
                    }
                }
            }
        </>
    }
}
