use super::*;

/// This is a utility for systemizing the site contents. The `content_html` property is in the
/// contents module of this directory.
/// In this way it's possible to put all the Function Components in this module folder and
/// re-export them to make the project more structured.
pub fn wrap_blog_subsite(nav: Navigator, content_html: Html) -> Html {
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
            {content_html}
        </>
    })
}
