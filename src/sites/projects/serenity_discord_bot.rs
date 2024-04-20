use super::*;

use crate::routes::Route;
use yew_router::prelude::*;

#[function_component(ProjectSerenityDiscordBot)]
pub fn serenity_discord_bot() -> Html {
    let nav = use_navigator().unwrap();

    let callback_nav = |target: Route| {
        let nav = nav.clone();
        Callback::from(move |_| nav.clone().push(&target))
    };
    wrap_site(html! {
        <>
            <br />
            <a href={"javascript:void(0);"} onclick={callback_nav(Route::Projects)}>
                {"Go back"}
            </a>
            <br />
            <h2>{"Serenity Discord Bot"}</h2>
            <br />
            {serenity_discord_bot_description()}
        </>
    })
}
