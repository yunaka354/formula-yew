use yew::prelude::*;

#[function_component(Spinner)]
pub fn spinner() -> Html {
    html! {
        <div class="border-t-transparent border-solid animate-spin rounded-full w-8 h-8 border-4 border-blue-400 border-t-4"></div>
    }
}
