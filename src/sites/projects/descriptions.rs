use super::*;

pub fn serenity_discord_bot_description() -> Html {
    html! {
        <>
            <br />
            <h2>{"Serenity Discord Bot"}</h2>
            <br />
            <a href="https://github.com/1Kill2Steal/serenity-discord-bot" target="_blank">
                {"GitHub repo"}
            </a>
            <br />
            <br />
            {" It uses the "}
            <a href="https://github.com/serenity-rs/poise" target="_blank">
                {"poise"}
            </a>
            {" and "}
            <a href="https://github.com/serenity-rs/serenity" target="_blank">
                {"serenity"}
            </a>
            {" frameworks in order to provide the necessary functionality for the bot."}
            <br />
            <br />
            {"Additional features:"}
            <br />
            <ul>
                <li>
                {"DataBase connections (w/ SQLite)."}
                </li>
                <li>
                {"Docker & DockerCompose integration."}
                </li>
                <li>
                {"Continuous Integration via GitHub Actions."}
                </li>
            </ul>
            <br />
        </>
    }
}
