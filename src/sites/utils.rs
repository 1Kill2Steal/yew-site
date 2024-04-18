use crate::data::{CONTENT, SITE_CONTENT};
use crate::templates::{footer::Footer, nav::Nav};
use yew::prelude::*;
/// A custom made wrapper function for the main page layout of my project which includes the Navbar
/// and Footer.
#[export_name = "wrap_site"]
pub fn wrap_site(content: Html) -> Html {
    html! {
        <root>
            <div class={CONTENT}>
                <Nav />
                <div class={SITE_CONTENT}>
                    { content }
                </div>
            </div>
            <Footer />
        </root>
    }
}
