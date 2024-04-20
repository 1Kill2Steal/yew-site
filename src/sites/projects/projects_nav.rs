use super::*;

use crate::routes::Route;
use yew_router::prelude::*;

const PROJECT_DETAILS_CLASS: &'static str = "project-details";

#[function_component(Projects)]
pub fn projects() -> Html {
    let nav = use_navigator().unwrap();

    let callback_nav = |target: Route| {
        let nav = nav.clone();
        Callback::from(move |_| nav.clone().push(&target))
    };
    wrap_site(html! {
        <>
            <h1>{ "Blog navigation" }</h1>
            <p>
                <a href={"javascript:void(0);"} onclick={callback_nav(Route::ProjectSerenityDiscordBot)}>
                    {"Serenity discord bot"}
                </a>
            </p>
            <details class={PROJECT_DETAILS_CLASS}>
                <summary>{"Description"}</summary>
                {serenity_discord_bot_description()}
            </details>
        </>
    })
}
