use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{home::Home, article::Article};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/article/:slug")]
    Article { slug: String }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html!{ <Home /> },
        Route::Article { slug } => html!{ <Article slug={slug.to_owned()} /> },
    }
}