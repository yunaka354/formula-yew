use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="
            bg-slate-900
            text-white
            text-center
            p-2
            shadow-md
            sticky
            top-0
            z-50
            flex
            justify-start
            items-center"
        >
            <a href="/">
                <img class="website-icon" src="static/icon.png" />
            </a>
            <div class="px-4">
                <h1 class="text-xl font-semibold">
                    <a href="/">{"Formula Yew"}</a>
                </h1>
            </div>
        </header>
    }
}
