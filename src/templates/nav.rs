use crate::routes::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html {
    let nav_homepage = use_navigator().unwrap();
    let nav_details = use_navigator().unwrap();
    let nav_gallery = use_navigator().unwrap();

    let callback_homepage = Callback::from(move |_| nav_homepage.push(&Route::Homepage));
    let callback_details = Callback::from(move |_| nav_details.push(&Route::Details));
    let callback_gallery = Callback::from(move |_| nav_gallery.push(&Route::Gallery));
    html! {
        <nav>
            <button onclick={callback_homepage}>{ "Homepage" }</button>
            <button onclick={callback_details}>{ "Details" }</button>
            <button onclick={callback_gallery}>{ "Gallery" }</button>
        </nav>
    }
}
