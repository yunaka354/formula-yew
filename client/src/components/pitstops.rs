use crate::components::{laps::LapsBox, Laps, Results, Standings};
use std::collections::HashMap;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Pitstops)]
pub fn pitstops() -> Html {
    let input_node_ref = use_node_ref();

    let onkeypress = {
        let input_node_ref = input_node_ref.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input_node) = input_node_ref.cast::<HtmlInputElement>() {
                    let value = input_node.value();
                    log::info!("input value: {}", value);
                }
            }
        })
    };
    html! {
        <>
            <div class={"text-center py-4"}> // Center the title and add vertical padding
                <h1 class={"text-2xl font-bold mb-4"}>{ "Pitstops" }</h1> // Add bottom margin to the title
            </div>
            <div>
                <input
                    type="text"
                    class="border border-gray-300 rounded-md p-2"
                    placeholder="Enter a year"
                    ref={input_node_ref}
                    {onkeypress}
                />
            </div>
        </>
    }
}
