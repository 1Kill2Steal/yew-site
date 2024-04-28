use super::*;

pub mod router;

use crate::routes::Route;

use crate::sites::{
    about_me::AboutMe,
    blog::{blog_nav::*, understanding_big_o_notation::*},
    gallery::Gallery,
    homepage::Homepage,
    projects::{projects_nav::Projects, *},
};

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Homepage => html! {
            <Homepage />
        },
        Route::AboutMe => html! {
            <AboutMe />
        },

        Route::Blog => html! {
            <Blog />
        },
        Route::BlogUnderstandingBigONotation => html! {
            <BlogUnderstandingBigONotation />
        },

        Route::Projects => html! {
            <Projects />
        },
        Route::ProjectSerenityDiscordBot => html! {
            <ProjectSerenityDiscordBot />
        },
        Route::ProjectDiscordInteractionsBot => html! {
            <ProjectDiscordInteractionsBot />
        },
        Route::ProjectCountingBlinks => html! {
            <ProjectCountingBlinks />
        },

        Route::Gallery => html! {
            <Gallery />
        },

        Route::NotFound => html! { <a align="center">{ "404 - Webpage Not Found" }</a> },
    }
}
