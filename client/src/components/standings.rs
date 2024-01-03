use crate::components::DetailProps;
use crate::components::Spinner;
use crate::components::StandingChart;
use crate::models::ChartData;
use crate::utils;
use yew::prelude::*;

#[function_component(Standings)]
pub fn standings(props: &DetailProps) -> Html {
    let props = props.clone();
    let data = use_state(|| None);

    {
        let data = data.clone();
        use_effect_with((), move |_| {
            let data = data.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let data = data.clone();
                let url = format!(
                    "http://localhost:3000/standings?year={}&round={}",
                    props.year, props.round
                );
                let response = utils::fetch_server::<ChartData<String, i32>>(&url).await;
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
                            <StandingChart chart_data={data} plot_id={"plot-standing"} />
                        }
                    }
                }
            }
        </>
    }
}
