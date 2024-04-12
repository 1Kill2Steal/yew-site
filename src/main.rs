mod data;
mod enums;
mod routes;
mod sites;
mod structs;
mod switch;
mod templates;
mod utils;

use switch::router::Main;
use yew::prelude::*;

fn main() {
    yew::Renderer::<Main>::new().render();
}
