use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let user = use_state(|| None);
    {
        let user = user.clone();
        use_effect_with((), move |_| {
            let user = user.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = "http://localhost:3000/user";
                let response: User = Request::get(url)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                user.set(Some(response));
            });
            || ()
        });
    }
    html! {
        <div>
            {
                match (*user).clone() {
                    Some(user) => {
                        html! { format!("id: {:?} username: {:?}", user.id, user.username) }
                    },
                    None => html!{ <p>{ "fetching..." }</p> }
                }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
}
