mod app;
mod pages;
mod components;
mod services;

use app::App;

fn main() {
    yew::start_app::<App>();
}
