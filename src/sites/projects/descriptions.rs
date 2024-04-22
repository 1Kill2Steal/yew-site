use super::*;

const PROJECT_SHOWCASE: &'static str = "project-showcase";

fn project_showcase_wrap(item: Html) -> Html {
    html! {
        <div class={PROJECT_SHOWCASE}>
            {item}
        </div>
    }
}

pub fn serenity_discord_bot_description() -> Html {
    let assets_folder = || String::from("/public/projects/serenity-discord-bot/");
    project_showcase_wrap(html! {
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
            <h4>{"Project showcase content"}</h4>
            <div class="project-videos">
                <video controls=true>
                    <source src={assets_folder() + "showcase-1.mp4"} type="video/mp4" />
                </video>
                <video controls=true>
                    <source src={assets_folder() + "showcase-2.mp4"} type="video/mp4" />
                </video>
                <video controls=true>
                    <source src={assets_folder() + "showcase-3.mp4"} type="video/mp4" />
                </video>
                <video controls=true>
                    <source src={assets_folder() + "showcase-4.mp4"} type="video/mp4" />
                </video>
            </div>
            <br />
        </>
    })
}

pub fn discord_interactions_bot_description() -> Html {
    let assets_folder = || String::from("/public/projects/discord-interactions-bot/");
    project_showcase_wrap(html! {
        <>
            <br />

            <h2>{"Discord Interactions Bot"}</h2>

            <br />
            <a href="https://github.com/1Kill2Steal/serenity-discord-bot" target="_blank">
                {"GitHub repo"}
            </a>
            <br />
            <br />

            {"It's written in TypeScript and it uses the "}
            <a href="https://discord.js.org/" target="_blank">
                {"discord.js"}
            </a>
            {" library."}

            <br />

            {"It was made for "}
            <a href="https://www.youtube.com/@nopengoo" target="_blank">
                {"Nopengoo"}
            </a>
            {", a content creator for Genshin Impact animations. Here's the invite link to "}
            <a href="https://discord.gg/smhQTCwaAF" target="_blank">
                {"the discord server"}
            </a>
            {"."}

            <br />
            <br />

            {"Additional features:"}
            <br />
            <ul>
                <li>
                {"Database handling with MongoDB."}
                </li>
            </ul>

            <br />

            <h4>{"Project showcase content"}</h4>

            <div class="project-videos">
                <video controls=true>
                    <source src={assets_folder() + "showcase-1.mp4"} type="video/mp4" />
                </video>
                <video controls=true>
                    <source src={assets_folder() + "showcase-2.mp4"} type="video/mp4" />
                </video>
                <video controls=true>
                    <source src={assets_folder() + "showcase-3.mp4"} type="video/mp4" />
                </video>
                <video controls=true>
                    <source src={assets_folder() + "showcase-4.mp4"} type="video/mp4" />
                </video>
            </div>

            <br />
        </>
    })
}

pub fn counting_blinks_description() -> Html {
    project_showcase_wrap(html! {
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
    })
}
