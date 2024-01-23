use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(Pitstops)]
pub fn pitstops() -> Html {
    let input_node_ref = use_node_ref();
    let error_message = use_state(|| None);

    let onkeypress = {
        let input_node_ref = input_node_ref.clone();
        let error_message = error_message.clone();

        Callback::from(move |e: KeyboardEvent| {
            // Process when user hit Enter
            if e.key() == "Enter" {
                if let Some(input_node) = input_node_ref.cast::<HtmlInputElement>() {
                    let value = input_node.value();
                    // Check if the input is a number
                    match value.parse::<u32>() {
                        Ok(year) => {
                            error_message.set(None); // Clear the error message

                            // Check if the year is between 2020 and 2023
                            if year >= 2020 && year <= 2023 {
                                log::info!("year: {}", year);
                            } else {
                                error_message.set(Some(
                                    "Please enter a year between 2020 and 2023".to_string(),
                                ));
                            }
                        }
                        // If the input is not a number, show an error message
                        Err(_) => error_message.set(Some("Please enter a number".to_string())),
                    }
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
                // Show the error message if there is one
                {
                    if let Some(error) = &*error_message.clone() {
                        html! {
                            <p class="text-red-500">{error}</p>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </>
    }
}
