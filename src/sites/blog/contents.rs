use super::*;

const BLOG_SHOWCASE: &'static str = "blog-showcase";
fn blog_showcase_wrap(item: Html) -> Html {
    html! {
        <div class={BLOG_SHOWCASE}>
            {item}
        </div>
    }
}

const ID_REF: &'static str = "#";
const METADATA: &'static str = "metadata";
const BLOG_LINK: &'static str = "https://1kill2steal.netlify.app/blog/";

fn blog_metadata(clipboard: yew_hooks::UseClipboardHandle, date: &str, link: &'static str) -> Html {
    let clipboard = clipboard.clone();
    let blog_link = || BLOG_LINK.to_owned() + link;
    let write_to_clipboard = || {
        let clipboard = clipboard.clone();
        Callback::from(move |_| clipboard.write_text(blog_link()))
    };
    html! {
        <div class={METADATA}>
            // Nerd fonts:
            // https://www.nerdfonts.com/cheat-sheet
            <span class={"nf-md-calendar_month"}
                  style={"margin-right: 10px;"}
            >
            </span>
            <i>{ date }</i>
            <br />
                <span class={"nf-md-link_variant"}
                      style={"margin-right: 10px;"}
                      onclick={write_to_clipboard()}
                >
                </span>
                <i onclick={write_to_clipboard()}>
                    { "Share" }
                </i>
        </div>
    }
}

fn blog_contents_navigation(items: Html) -> Html {
    html! {
        <details>
            <summary>{"Contents"}</summary>
            <div class={"blog-contents-items"}>
                {items}
            </div>
        </details>
    }
}

pub fn blog_understaning_big_o_notation_contents(clipboard: yew_hooks::UseClipboardHandle) -> Html {
    let clipboard = clipboard.clone();
    let link = "understanding-big-o-notation/";
    let introduction = "introduction";
    let contents = html! {
        <>
            <a href={ID_REF.to_owned() + introduction}>
                {"Introduction"}
            </a>
        </>
    };
    blog_showcase_wrap(html! {
        <>
            <h1>{ "Understanding Big O Notation" }</h1>
            {blog_metadata(
                clipboard,
                "2024-04-23",
                link
            )}
            {blog_contents_navigation(contents)}
            <h2 id={introduction}>
                {"Introduction"}
            </h2>
            <p>
                <a href="https://en.wikipedia.org/wiki/Big_O_notation"
                   target="_blank">
                    {"Big O Notation"}
                </a>
                {" is a mathematical notation that describes the behavior of a value (most commonly
                denoted as 'n') as it reaches infinity. That's how you'd see it explained in most places"}
            </p>
        </>
    })
}
