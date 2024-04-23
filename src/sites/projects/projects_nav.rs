use super::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    let nav = use_navigator().unwrap();

    let callback_nav = |target: Route| {
        let nav = nav.clone();
        Callback::from(move |_| nav.clone().push(&target))
    };
    wrap_site(html! {
        <>
            <h1>{ "Projects" }</h1>
            <div tag={"projects-list"} class={"projects-list"}>
                <div tag={"serenity_discord_bot"} class={"project-item"}>
                    <p>
                        <a href={"javascript:void(0);"} onclick={callback_nav(Route::ProjectSerenityDiscordBot)}>
                            {"Serenity discord bot"}
                        </a>
                    </p>
                </div>
                <div tag={"discord_interactions_bot"} class={"project-item"}>
                    <p>
                        <a href={"javascript:void(0);"} onclick={callback_nav(Route::ProjectDiscordInteractionsBot)}>
                            {"Discord interactions bot"}
                        </a>
                    </p>
                </div>
                <div tag={"counting_blinks"} class={"project-item"}>
                    <p>
                        <a href={"javascript:void(0);"} onclick={callback_nav(Route::ProjectCountingBlinks)}>
                            {"Counting Blinks"}
                        </a>
                    </p>
                </div>
            </div>
        </>
    })
}
