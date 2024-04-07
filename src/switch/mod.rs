pub mod router;

use crate::routes::Route;

use crate::sites::details::Details;
use crate::sites::gallery::Gallery;
use crate::sites::homepage::Homepage;

use yew::prelude::*;

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Homepage => html! {
            <Homepage />
        },
        Route::Details => html! {
            <Details />
        },
        Route::Gallery => html! {
            <Gallery />
        },
        Route::NotFound => html! { <a align="center">{ "404 - Webpage Not Found" }</a> },
    }
}
