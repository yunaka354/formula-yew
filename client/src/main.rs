#[allow(unused_imports)]
use log::info;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod models;
mod utils;
use components::{Detail, Races, Seasons};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/races")]
    Races,
    #[at("/")]
    Seasons,
    #[at("/results")]
    Results,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Races => html! { <Races /> },
        Route::Seasons => html! { <Seasons /> },
        Route::Results => html! { <Detail /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
