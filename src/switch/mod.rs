use super::*;

pub mod router;

use crate::routes::Route;

use crate::sites::about_me::AboutMe;
use crate::sites::gallery::Gallery;
use crate::sites::homepage::Homepage;

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Homepage => html! {
            <Homepage />
        },
        Route::AboutMe => html! {
            <AboutMe />
        },
        Route::Gallery => html! {
            <Gallery />
        },
        Route::NotFound => html! { <a align="center">{ "404 - Webpage Not Found" }</a> },
    }
}
