mod app;
mod components;
mod routes;
mod page;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
