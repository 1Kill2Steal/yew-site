use super::*;

use crate::templates::footer::Footer;
use crate::templates::nav::Nav;

#[function_component(AboutMe)]
pub fn details() -> Html {
    html! {
        <root>
            <div class={CONTENT}>
                <Nav />
                <div class={SITE_CONTENT}>
                    <h1>{ "My name is Ivan" }</h1>

                    <h2>{"Project lore"}</h2>

                    <p>
                        {"I'm mainly a back-end developer but with "}
                        <a href="https://github.com/1Kill2Steal/yew-site" target="_blank">
                            {"this project"}
                        </a>
                        {" I decided to challenge myself and try out something new."}

                        <br />

                        {"Let's just say... It was a bit of a pain to get this working to a
                        somewhat decent level. Not to mention that a lot of functionality like
                        closing the gallery preview images with esc and/or clicking outside
                        of the image isn't implemented because I'm not exactly sure how to
                        do it. Anyway."}

                        <br />
                    </p>

                    <h2>{"Hobbies"}</h2>

                    <ul>
                        <li>
                        {"Linux (imagine having an OS as a hobby)"}
                        </li>
                        <li>
                        {"Programming (and I mainly do it in Rust)"}
                        </li>
                        <li>
                        {"Anime (I don't watch it a lot but I like it)"}
                        </li>
                        <li>
                        {"Sleeping- okay that's not a hobby-"}
                        </li>
                    </ul>

                    <h2>{"Other projects"}</h2>
                    <ul>
                        <li>
                            <a href="https://github.com/1Kill2Steal/serenity-discord-bot"
                               target="_blank"
                            >
                                {"Discord bot in Rust"}
                            </a>
                        </li>
                        <li>
                            <a href="https://github.com/1Kill2Steal/discord-interactions-bot"
                               target="_blank"
                            >
                                {"Discord bot in TypeScript"}
                            </a>
                            {" (Yes, I don't only do Rust)"}
                        </li>
                        <li>
                            <a href="https://github.com/1Kill2Steal/leetcode-trees-rs"
                               target="_blank"
                            >
                                {"LeetCode mini library in Rust"}
                            </a>
                        </li>
                        <li>
                            <a href="https://github.com/1Kill2Steal/hunger-games-website"
                               target="_blank"
                            >
                                {"Hunger Games Website with TypeScript"}
                            </a>
                        </li>
                        <li>
                            {"More on my "}
                            <a href="https://github.com/1Kill2Steal" target="_blank">
                                {"GitHub profile"}
                            </a>
                        </li>
                    </ul>

                </div>
            </div>
            <Footer />
        </root>
    }
}
