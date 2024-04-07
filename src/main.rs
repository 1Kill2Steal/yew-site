mod routes;
mod sites;
mod switch;
mod templates;

use switch::router::Main;

fn main() {
    yew::Renderer::<Main>::new().render();
}
