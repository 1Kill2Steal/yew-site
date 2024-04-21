use super::*;

/// This is a utility for systemizing the site contents. The `description_html` property is in the
/// descriptions module of this directory.
/// In this way it's possible to put all the Function Components in this module folder and
/// re-export them to make the project more structured.
pub fn wrap_project_subsite(nav: Navigator, description_html: Html) -> Html {
    let callback_nav = |target: Route| {
        let nav = nav.clone();
        Callback::from(move |_| nav.clone().push(&target))
    };
    wrap_site(html! {
        <>
            <br />
            <a href={"javascript:void(0);"} onclick={callback_nav(Route::Projects)}>
                {"Go back"}
            </a>
            {description_html}
        </>
    })
}
