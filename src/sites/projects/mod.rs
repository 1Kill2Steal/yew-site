use super::*;

pub mod descriptions;
pub mod projects_nav;
pub mod utils;

// Re-exports for convenience sake.
use crate::sites::projects::descriptions::*;
use crate::sites::projects::utils::*;

use crate::routes::Route;
use yew_router::prelude::*;

#[function_component(ProjectSerenityDiscordBot)]
pub fn serenity_discord_bot() -> Html {
    let nav = use_navigator().unwrap();
    wrap_project_subsite(nav, serenity_discord_bot_description())
}

#[function_component(ProjectDiscordInteractionsBot)]
pub fn discord_interactions_bot() -> Html {
    let nav = use_navigator().unwrap();
    wrap_project_subsite(nav, discord_interactions_bot_description())
}

#[function_component(ProjectCountingBlinks)]
pub fn counting_blinks() -> Html {
    let nav = use_navigator().unwrap();
    wrap_project_subsite(nav, counting_blinks_description())
}
