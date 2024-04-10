mod data;
mod routes;
mod sites;
mod switch;
mod templates;
mod utils;

use switch::router::Main;
use yew::prelude::*;

fn main() {
    yew::Renderer::<Main>::new().render();
}
