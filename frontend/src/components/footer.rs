use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html!{        
        <footer>
            <div class="container">
                <a href="/" class="logo-font">{"conduit"}</a>
                <span class="attribution">
                {"An interactive learning project from "}<a href="https://thinkster.io">{"Thinkster"}</a>{". Code"} {&'\u{0026}'}{" design licensed under MIT."}
                </span>
            </div>
        </footer>
    }
}