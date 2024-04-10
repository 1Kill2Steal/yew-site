use super::*;

use crate::routes::Route;

use yew_router::prelude::*;

#[function_component(Main)]
pub fn main() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
