use super::*;

use crate::routes::Route;
use yew_router::prelude::*;

#[function_component(BlogTest)]
pub fn blog_test() -> Html {
    let nav = use_navigator().unwrap();

    let callback_nav = |target: Route| {
        let nav = nav.clone();
        Callback::from(move |_| nav.clone().push(&target))
    };
    wrap_site(html! {
        <>
            <br />
            <a href={"javascript:void(0);"} onclick={callback_nav(Route::Blog)}>
                {"Go back"}
            </a>
            <h1>{ "Test blog" }</h1>
            <p>
                {"This content is a Work In Progress..."}
            </p>
        </>
    })
}
