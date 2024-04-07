use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
            <footer>
                <h2>
                    { "By " }
                    <a href="https://github.com/1kill2steal" target="_blank">
                        { "1Kill2Steal" }
                    </a>
                </h2>
                <p>
                    {
                        "Webpage is licensed under the MIT License because I know no one will even look at it anyway."
                    }
                </p>
            </footer>
    }
}
