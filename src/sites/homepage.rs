use super::*;

use crate::routes::Route;
use yew_router::prelude::*;

#[function_component(Homepage)]
pub fn homepage() -> Html {
    let nav_gallery = use_navigator().unwrap();
    let callback_gallery = Callback::from(move |_| nav_gallery.push(&Route::Gallery));
    wrap_site(html! {
        <>
            <h1>{ "What's this site?" }</h1>
            <p>
                {"This is a simple site which uses Yew.rs in order to
                    display its content. Additionally, it uses the
                    yew-router crate in order to achieve the
                    navigation functionality."}

                <br />

                {"No one cares about that though... Make sure to check out the best part of this site! ("}
                <a href={"javascript:void(0);"} onclick={callback_gallery}>
                {"the Hu Tao gallery"}
                </a>
                {")"}
            </p>
        </>
    })
}
