#[allow(unused_imports)]
use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod models;
mod utils;
use components::{Detail, Header, Pitstops, Races, Seasons};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/races")]
    Races,
    #[at("/")]
    Seasons,
    #[at("/results")]
    Results,
    #[at("/pitstops")]
    Pitstops,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Races => html! { <Races /> },
        Route::Seasons => html! { <Seasons /> },
        Route::Results => html! { <Detail /> },
        Route::Pitstops => html! { <Pitstops /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header />
            <div class={"wrapper flex bg-gray-100 justify-center items-center min-h-screen p-10"}>
                <div class={"container mx-auto bg-white rounded-lg shadow p-6 max-w-7xl"}>
                    <BrowserRouter>
                        <Switch<Route> render={switch} />
                    </BrowserRouter>
                </div>
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
