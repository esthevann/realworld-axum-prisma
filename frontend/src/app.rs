use yew::prelude::*;

use crate::pages::home::Home;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <Home />
    }
}