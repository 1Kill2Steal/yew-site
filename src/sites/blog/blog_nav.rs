use super::*;

use crate::routes::Route;
use yew_router::prelude::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    let nav = use_navigator().unwrap();

    let callback_nav = |target: Route| {
        let nav = nav.clone();
        Callback::from(move |_| nav.clone().push(&target))
    };
    wrap_site(html! {
        <>
            <h1>{ "Blog navigation" }</h1>
            <p>
                {"This content is a Work In Progress..."}
                <br />
                <a href={"javascript:void(0);"} onclick={callback_nav(Route::BlogTest)}>
                    {"Test blog"}
                </a>
            </p>
        </>
    })
}
