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

pub fn counting_blinks_description() -> Html {
    html! {
        <>
            <br />
            <h2>{"Counting Blinks"}</h2>
            <br />
            <a href="https://github.com/1Kill2Steal/counting-blinks" target="_blank">
                {"GitHub repo"}
            </a>
            <br />
            <br />
            {"This is a small project and it really doesn't provide a whole lot of unique stuff. I
            made it to test out how it's like to develop programs for Arduinos in Rust (It's
            actually quite convenient). It uses "}
            <a href="https://github.com/Rahix/avr-hal/tree/main/arduino-hal" target="_blank">
                {"this hal"}
            </a>
            {" (Hardware Abstraction Layer)"}
            <br />
            {"A convenient feature that's provided in it is the ability to generate a template
            project for your own Arduino device."}
            <br />
            {"It depends on "}
            <a href="https://github.com/Rahix/avr-hal/tree/main/ravedude" target="_blank">
                {"ravedude"}
            </a>
            {" which was made by the same developer and the compilation is done with "}
            <a href="https://gcc.gnu.org/wiki/avr-gcc" target="_blank">
                {"gcc-avr"}
            </a>
            {"."}
            <br />
            <br />
            <img src="/public/projects/counting-blinks.gif" />
            <br />
        </>
    }
}
