use super::*;

fn blog_showcase_wrap(item: Html) -> Html {
    html! {
        <div class={BLOG_SHOWCASE}>
            {item}
        </div>
    }
}

pub fn blog_metadata(
    clipboard: yew_hooks::UseClipboardHandle,
    date: &str,
    reading_time: &str,
    link: &'static str,
) -> Html {
    let clipboard = clipboard.clone();
    let blog_link = || BLOG_LINK.to_owned() + link;
    let write_to_clipboard = || {
        let clipboard = clipboard.clone();
        Callback::from(move |_| clipboard.write_text(blog_link()))
    };
    html! {
        <div class={BLOG_METADATA}
             style={"width: 100vw;"}>
            // Nerd fonts:
            // https://www.nerdfonts.com/cheat-sheet
            <p>
                <i class={"nerd-font-glyph nf-md-calendar_month"}>
                    { date }
                </i>
            </p>

            <p>
                <i class={"nerd-font-glyph nf-fa-book_open_reader"}>
                    { reading_time }
                </i>
            </p>

            <p>
                <i class={"share-button nerd-font-glyph nf-md-link_variant"}
                   onclick={write_to_clipboard()}
                >
                    { "Share" }
                </i>
            </p>
        </div>
    }
}

fn set_section_items(items: Vec<&str>) -> Html {
    html! {
        <>
        {
            items.iter().map(|item| {
                html!{
                    <div class={BLOG_DETAILS_ITEM}>
                        <a href={String::from('#') + item}
                        class={DETAILS_ITEM_GLYPH}
                        >
                            {item}
                        </a>
                    </div>
                }
            }).collect::<Html>()
        }
        </>
    }
}

/// This is a utility for systemizing the site contents. The `content_html` property is in the
/// contents module of this directory.
/// In this way it's possible to put all the Function Components in this module folder and
/// re-export them to make the project more structured.
pub fn wrap_blog_subsite(
    nav: Navigator,
    clipboard: yew_hooks::UseClipboardHandle,
    date: &'static str,
    reading_time: &'static str,
    link: &'static str,
    blog_name: &str,
    blog_contents_items: Vec<&str>,
    content_html: Html,
) -> Html {
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
            {blog_showcase_wrap(
                html! {
                    <>
                        <h1>{blog_name}</h1>

                        {blog_metadata(
                            clipboard,
                            &date,
                            &reading_time,
                            link
                        )}

                        <br />

                        {blog_contents_navigation(
                            set_section_items(
                                blog_contents_items
                            )
                        )}

                        <br />

                        <div class={"blog-showcase-content"}>
                            {content_html}
                        </div>
                    </>
                }
            )}
        </>
    })
}

pub fn blog_contents_navigation(items: Html) -> Html {
    html! {
        <details>
            <summary>{"Contents"}</summary>
            <br />
            <div class={"blog-contents-items"}>
                {items}
            </div>
        </details>
    }
}
