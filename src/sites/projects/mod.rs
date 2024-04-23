use super::*;

pub mod contents;
pub mod projects_nav;
pub mod utils;

// Re-exports for convenience sake.
use {contents::*, utils::*};

#[function_component(ProjectSerenityDiscordBot)]
pub fn serenity_discord_bot() -> Html {
    let nav = use_navigator().unwrap();
    wrap_project_subsite(nav, serenity_discord_bot_contents())
}

#[function_component(ProjectDiscordInteractionsBot)]
pub fn discord_interactions_bot() -> Html {
    let nav = use_navigator().unwrap();
    wrap_project_subsite(nav, discord_interactions_bot_contents())
}

#[function_component(ProjectCountingBlinks)]
pub fn counting_blinks() -> Html {
    let nav = use_navigator().unwrap();
    wrap_project_subsite(nav, counting_blinks_contents())
}
