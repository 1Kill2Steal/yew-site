use crate::templates::footer::Footer;
use crate::templates::nav::Nav;

use yew::functional::*;
use yew::prelude::*;

#[function_component(Homepage)]
pub fn homepage() -> Html {
    html! {
        <root>
            <div class="content">
                <Nav />
                <h1>{ "What's this site?" }</h1>
                <p>
                    {"This is a simple site which uses Yew.rs in order to
                    display its content. Additionally, it uses the
                    yew-router crate in order to achieve the
                    navigation functionality."}
                </p>
            </div>
            <Footer />
        </root>
    }
}
