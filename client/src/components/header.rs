use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="bg-slate-900 text-white text-center p-4 shadow-md">
            <nav class="navbar">
                <h1 class="text-xl font-semibold">
                    <a href="/">{"Formula Yew"}</a>
                </h1>
            </nav>
        </header>
    }
}
