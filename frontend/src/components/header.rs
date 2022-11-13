use yew::prelude::*;
use yew_router::prelude::Link;

use crate::app::Route;

#[function_component(Header)]
pub fn header() -> Html {
    html!{
        <nav class="navbar navbar-light">
    <div class="container">
        <a class="navbar-brand" href="index.html">{"conduit"}</a>
        <ul class="nav navbar-nav pull-xs-right">
            <li class="nav-item">
                // <!-- Add "active" class when you're on that page" -->
                <Link<Route>  to={Route::Home}>
                    <p class="nav-link active">{"Home"}</p>
                </Link<Route>>
            </li>
            <li class="nav-item">
                <Link<Route> to={Route::CreateArticle}>
                    <p class="nav-link">
                        <i class="ion-compose"></i>{&'\u{00a0}'}{"New Article"}
                    </p>
                </Link<Route>>
            </li>
            <li class="nav-item">
                <Link<Route>  to={Route::Settings}>
                    <p class="nav-link">
                        <i class="ion-gear-a"></i>{&'\u{00a0}'}{"Settings"}
                    </p>             
                </Link<Route>>
            </li>
            <li class="nav-item">
                <Link<Route>  to={Route::Login}>
                    <p class="nav-link">{"Sign in"}</p>
                </Link<Route>>
            </li>
            <li class="nav-item">
                <Link<Route>  to={Route::Register}>
                    <p class="nav-link">{"Sign up"}</p>
                </Link<Route>>
            </li>
        </ul>
    </div>
</nav>
    }
}