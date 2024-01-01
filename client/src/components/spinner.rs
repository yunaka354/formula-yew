use yew::prelude::*;

#[function_component(Spinner)]
pub fn spinner() -> Html {
    html! {
        <div class="flex justify-center items-center">
            <div class="animate-spin rounded-full h-32 w-32 border-t-2 border-b-2 border-gray-900"></div>
        </div>
    }
}
