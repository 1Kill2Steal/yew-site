use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let mobile_nav = use_state(|| true);
    let flip = || {
        let item = mobile_nav.clone();
        Callback::from(move |_| item.set(!*item))
    };
    let opened_nav_item = "mobile-nav-item";
    let set_nav_item_class = || {
        if *mobile_nav {
            "nav-item"
        } else {
            opened_nav_item
        }
    };

    let opened_nav_container = "opened-nav-container";
    let set_nav_container = || {
        if *mobile_nav {
            "nav-container"
        } else {
            opened_nav_container
        }
    };

    let opened_nav_button = "opened-mobile-nav-button";
    let set_nav_button = || {
        if *mobile_nav {
            "mobile-nav-button"
        } else {
            opened_nav_button
        }
    };

    let opened_nav = "opened-nav";
    let set_nav = || {
        if *mobile_nav {
            "nav"
        } else {
            opened_nav
        }
    };

    let nav = use_navigator().unwrap();

    let callback_nav = |target: Route| {
        let nav = nav.clone();
        Callback::from(move |_| nav.clone().push(&target))
    };

    html! {
        <nav class={classes!{set_nav()}}>
            <div key={"mobile_nav"} class={classes!{set_nav_button()}} onclick={flip()}>
                <a href={"javascript:void(0);"}>{ "☰" }</a>
            </div>
            <div key={"nav_container"} class={classes!{set_nav_container()}}>
                <div key={"homepage"} class={classes!{set_nav_item_class()}} onclick={callback_nav(Route::Homepage)}>
                    <a href={"javascript:void(0);"}>{ "Homepage" }</a>
                </div>
                <div key={"about_me"} class={classes!{set_nav_item_class()}} onclick={callback_nav(Route::AboutMe)}>
                    <a href={"javascript:void(0);"}>{ "About Me" }</a>
                </div>
                <div key={"gallery"} class={classes!{set_nav_item_class()}} onclick={callback_nav(Route::Gallery)}>
                    <a href={"javascript:void(0);"}>{ "Gallery" }</a>
                </div>
            </div>
        </nav>
    }
}
