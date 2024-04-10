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
    // You may add a name to it if you want some special styling. These differences in the classes
    // were meant to be used to handle the following edge scenario:
    // - User opened inspect mode -> sized down the window (to reveal the mobile nav)
    // - Then upscaled the window and forgot to reset the above flip state.
    // It's really whatever but decided to be perfectionistic so yea.
    let opened_nav_item = "";
    let set_nav_item_class = || {
        if *mobile_nav {
            "nav-item"
        } else {
            opened_nav_item
        }
    };

    let nav_homepage = use_navigator().unwrap();
    let nav_details = use_navigator().unwrap();
    let nav_gallery = use_navigator().unwrap();

    let callback_homepage = Callback::from(move |_| nav_homepage.push(&Route::Homepage));
    let callback_details = Callback::from(move |_| nav_details.push(&Route::Details));
    let callback_gallery = Callback::from(move |_| nav_gallery.push(&Route::Gallery));

    html! {
        <nav>
            <div key={"mobile_nav"} class={"mobile-nav"} onclick={flip()}>
                <a href={"javascript:void(0);"}>{ "☰" }</a>
            </div>
            <div key={"homepage"} class={classes!{set_nav_item_class()}} onclick={callback_homepage}>
                <a href={"javascript:void(0);"}>{ "Homepage" }</a>
            </div>
            <div key={"details"} class={classes!{set_nav_item_class()}} onclick={callback_details}>
                <a href={"javascript:void(0);"}>{ "Details" }</a>
            </div>
            <div key={"gallery"} class={classes!{set_nav_item_class()}} onclick={callback_gallery}>
                <a href={"javascript:void(0);"}>{ "Gallery" }</a>
            </div>
        </nav>
    }
}
