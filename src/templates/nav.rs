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
    let nav = String::from("nav");
    let nav_container = String::from("nav-container");
    let mobile_nav_button = String::from("mobile-nav-button");
    let nav_item = String::from("nav-item");
    let nav_section = String::from("nav-section");
    let section_navigation_items = String::from("section-nav-items");
    let opened = || String::from("opened-");
    let mobile = || String::from("mobile-");
    // let opened_mobile = || String::from("opened-mobile-");
    let (
        set_nav,
        set_nav_container,
        set_nav_button,
        set_nav_item_class,
        set_nav_section_class,
        set_nav_section_items,
    ) = {
        if *mobile_nav {
            (
                nav,
                nav_container,
                mobile_nav_button,
                nav_item,
                nav_section,
                section_navigation_items,
            )
        } else {
            (
                opened() + &nav,
                opened() + &nav_container,
                opened() + &mobile_nav_button,
                mobile() + &nav_item,
                mobile() + &nav_section,
                mobile() + &section_navigation_items,
            )
        }
    };

    let nav = use_navigator().unwrap();

    let callback_nav = |target: Route| {
        let nav = nav.clone();
        Callback::from(move |_| nav.clone().push(&target))
    };

    html! {
        <nav class={&set_nav}>
            <div key={"mobile_nav"} class={&set_nav_button} onclick={flip()}>
                <a href={"javascript:void(0);"}>{ "☰" }</a>
            </div>
            <div key={"nav_container"} class={&set_nav_container}>
                <div key={"homepage"} class={&set_nav_item_class} onclick={callback_nav(Route::Homepage)}>
                    <a href={"javascript:void(0);"}>{ "Homepage" }</a>
                </div>
                <div key={"about_me"} class={&set_nav_item_class} onclick={callback_nav(Route::AboutMe)}>
                    <a href={"javascript:void(0);"}>{ "About Me" }</a>
                </div>
                <details key={"blog_details"} class={&set_nav_section_class}>
                    <summary>{"Blog"}</summary>
                    <div key={"blog_navigation"} class={&set_nav_section_items}>
                        <a href={"javascript:void(0);"} onclick={callback_nav(Route::Blog)}>{ "All Blogs" }</a>
                        <br />
                        <a href={"javascript:void(0);"} onclick={callback_nav(Route::BlogTest)}>{ "Test Blog" }</a>
                    </div>
                </details>
                <details key={"projects_details"} class={&set_nav_section_class}>
                    <summary>{"Projects"}</summary>
                    <div key={"projects_navigation"} class={&set_nav_section_items}>
                        <a href={"javascript:void(0);"} onclick={callback_nav(Route::Projects)}>{ "All Projects" }</a>
                        <br />
                        <a href={"javascript:void(0);"} onclick={callback_nav(Route::ProjectSerenityDiscordBot)}>{ "Serenity Discord Bot" }</a>
                        <br />
                        <a href={"javascript:void(0);"} onclick={callback_nav(Route::ProjectDiscordInteractionsBot)}>{ "Discord Interactions Bot" }</a>
                        <br />
                        <a href={"javascript:void(0);"} onclick={callback_nav(Route::ProjectCountingBlinks)}>{ "Counting Blinks" }</a>
                    </div>
                </details>
                <div key={"gallery"} class={&set_nav_item_class} onclick={callback_nav(Route::Gallery)}>
                    <a href={"javascript:void(0);"}>{ "Gallery" }</a>
                </div>
            </div>
        </nav>
    }
}
