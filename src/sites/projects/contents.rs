use crate::utils::set_iframe_gist;

use super::*;

const PROJECT_SHOWCASE: &str = "project-showcase";
fn project_showcase_wrap(item: Html) -> Html {
    html! {
        <div class={PROJECT_SHOWCASE}>
            {item}
        </div>
    }
}

pub fn serenity_discord_bot_contents() -> Html {
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

pub fn discord_interactions_bot_contents() -> Html {
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

pub fn counting_blinks_contents() -> Html {
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

pub fn leetcode_trees_contents() -> Html {
    project_showcase_wrap(html! {
        <>
            <br />
            <h2>{"LeetCode Trees"}</h2>
            <br />
            <a href="https://github.com/1Kill2Steal/leetcode-trees-rs" target="_blank">
                {"GitHub repo"}
            </a>
            <br />
            <br />
            <a href="https://docs.rs/leetcode-trees-rs/latest/leetcode_trees_rs/" target="_blank">
                {"Docs.rs"}
            </a>
            <br />
            <br />
            {"A Rust library for Binary Trees in "}
            <a href="https://leetcode.com/" target="_blank">
                {"LeetCode"}
            </a>
            {". It uses the same struct signatures as the LeetCode (LC) problems (refer to: "}
            <a href="https://leetcode.com/problems/same-tree/" target="_blank">
                {"100. Same Tree"}
            </a>
            {" and "}
            <a href="https://leetcode.com/problems/binary-tree-level-order-traversal/" target="_blank">
                {"103. Binary Tree Level Order Traversal"}
            </a>
            {") but it expands upon their general implementations with additional macros as well as
            a proper"}<b>{"Result<T, E>"}</b>{" type which can be used in tests and/or your main
            function."}
            <br />
            <br />
            {"Feel free to check out the "}
            <a href="https://github.com/1Kill2Steal/leetcode-trees-rs/tree/main/solutions" target="_blank">
                {"Example Solutions"}
            </a>
            {" on GitHub."}
            <br />
            <br />
            {"Also feel free to check out the following example which is also available on the
            homepage of the GitHub repo: "}
            {set_iframe_gist("https://gist.github.com/1Kill2Steal/0ffdcc6e9defbcd0f1ca37b0cbe73a40", Some(1000))}
        </>
    })
}
