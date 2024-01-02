use crate::components::table::Table;
use crate::components::table::TableContents;
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
                        let contents = TableContents::convert_race_result(results);
                        html! {
                            <>
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
