use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Article, CreateEditArticle, Home, Profile, LoginRegister, Settings};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/article/:slug")]
    Article { slug: String },
    #[at("/login")]
    Login,
    #[at("/register")]
    Register,
    #[at("/profile/:username")]
    Profile { username: String },
    #[at("/editor/:slug")]
    EditArticle { slug: String },
    #[at("/editor")]
    CreateArticle,
    #[at("/settings")]
    Settings
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Article { slug } => html! { <Article slug={slug.to_owned()} /> },
        Route::Login => html! { <LoginRegister is_login={true}/> },
        Route::Register => html! { <LoginRegister is_login={false}/> },
        Route::Profile { username } => html!{ <Profile username={username.to_owned()} /> },
        Route::EditArticle { slug } => html!{ <CreateEditArticle is_edit={true} slug={Some(slug.to_owned())} /> },
        Route::CreateArticle => html!{ <CreateEditArticle is_edit={false} slug={Option::<String>::None} /> },
        Route::Settings => html!{ <Settings /> },
    }
}