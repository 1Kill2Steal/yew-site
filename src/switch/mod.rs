use super::*;

pub mod router;

use crate::routes::Route;

use crate::sites::{
    about_me::AboutMe,
    blog::{blog_nav::Blog, blog_test::BlogTest},
    gallery::Gallery,
    homepage::Homepage,
    projects::{projects_nav::Projects, serenity_discord_bot::ProjectSerenityDiscordBot},
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
        Route::BlogTest => html! {
            <BlogTest />
        },

        Route::Projects => html! {
            <Projects />
        },
        Route::ProjectSerenityDiscordBot => html! {
            <ProjectSerenityDiscordBot />
        },

        Route::Gallery => html! {
            <Gallery />
        },
        Route::NotFound => html! { <a align="center">{ "404 - Webpage Not Found" }</a> },
    }
}
