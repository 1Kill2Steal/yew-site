use super::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    let nav = use_navigator().unwrap();

    let callback_nav = |target: Route| {
        let nav = nav.clone();
        Callback::from(move |_| nav.clone().push(&target))
    };
    let project_item = |tag: &'static str, target: Route, text: &'static str| -> Html {
        html! {
            <div tag={tag} class={"project-item"}>
                <p>
                    <a href={"javascript:void(0);"} onclick={callback_nav(target)}>
                        {text}
                    </a>
                </p>
            </div>
        }
    };
    wrap_site(html! {
        <>
            <h1>{ "Projects" }</h1>
            <div tag={"projects-list"} class={"projects-list"}>
                {project_item("serenity_discord_bot", Route::ProjectSerenityDiscordBot, "Serenity discord bot")}
                {project_item("discord_interactions_bot", Route::ProjectDiscordInteractionsBot, "Discord Interactions Bot")}
                {project_item("counting_blinks", Route::ProjectCountingBlinks, "Counting Blinks")}
            </div>
        </>
    })
}
