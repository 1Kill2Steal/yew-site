use super::*;

use crate::templates::footer::Footer;
use crate::templates::nav::Nav;

#[function_component(Details)]
pub fn details() -> Html {
    html! {
        <root>
            <div class="content">
                <Nav />
                <h1>{ "Hello Nerds!" }</h1>
                <p>
                    {
                        "There's more technical stuff here (VIEWER DISCRETION IS STRONGLY
                        ADVISED!). Anyway, if you cloned this repo, chances are you're a nerd
                        anyway. <3"
                    }

                    <br />

                    {
                        "This repo was meant as a test in making a "
                    }
                    <a href={"https://en.wikipedia.org/wiki/Single-page_application"}
                       target={"_blank"}>
                        {"SPA"}
                    </a>
                    {
                        " (Single Page Application). The most appealing part of it is the gallery.
                        What I can say from my experience is that... I prefer back-end development.
                        Despite that, working with this framework was a fun experience and I got to
                        see a different perspective on development which opened my mind to new
                        stuff!"
                    }

                    <br />

                    {"Previously I had tried out "}
                    <a href={"https://rocket.rs"} target={"_blank"}>
                        {"Rocket.rs"}
                    </a>
                    {" in "}
                    <a href={"https://github.com/1Kill2Steal/rocket-htmx-site"} target={"_blank"}>
                        {"this project"}
                    </a>
                    {
                        " and routing there felt a bit more off. Maybe I was also discouraged
                        because I was newer to the Rust programming language as well. When I used
                        Rocket.rs however I had a decent time doing the back-end stuff (like
                        setting up the SQLite database) but as expected, the front-end part was
                        harder to do."
                    }
                </p>
            </div>
            <Footer />
        </root>
    }
}
