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
const TEX_ITEM_WIDTH: &'static str = "var(--tex-item-width)";

fn blog_metadata(clipboard: yew_hooks::UseClipboardHandle, date: &str, link: &'static str) -> Html {
    let clipboard = clipboard.clone();
    let blog_link = || BLOG_LINK.to_owned() + link;
    let write_to_clipboard = || {
        let clipboard = clipboard.clone();
        Callback::from(move |_| clipboard.write_text(blog_link()))
    };
    html! {
        <div class={METADATA}
             style={"width: 100vw;"}>
            // Nerd fonts:
            // https://www.nerdfonts.com/cheat-sheet
            <p>
                <i class={"nerd-font-glyph nf-md-calendar_month"}
                    style={"margin-right: 10px;"}
                >
                </i>
                <i>{ date }</i>
            </p>

            <p>
                <i class={"nerd-font-glyph nf-md-link_variant"}
                        style={"margin-right: 10px;"}
                        onclick={write_to_clipboard()}
                >
                </i>
                <i onclick={write_to_clipboard()}>
                    { "Share" }
                </i>
            </p>
        </div>
    }
}

fn blog_contents_navigation(items: Html) -> Html {
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

pub fn blog_understaning_big_o_notation_contents(clipboard: yew_hooks::UseClipboardHandle) -> Html {
    let clipboard = clipboard.clone();
    let link = "understanding-big-o-notation/";
    let introduction = "Introduction";
    let dictionary = "Dictionary";
    let analysis = "Analysis";
    let contents = html! {
        <>
            <a href={ID_REF.to_owned() + introduction}>
                {introduction}
            </a>
            <br />
            <a href={ID_REF.to_owned() + dictionary}>
                {dictionary}
            </a>
            <br />
            <a href={ID_REF.to_owned() + analysis}>
                {analysis}
            </a>
            <br />
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
            <br />
            {blog_contents_navigation(contents)}
            <br />

            <h2 id={introduction}>
                {introduction}
            </h2>

            <p>
                <a href="https://en.wikipedia.org/wiki/Big_O_notation"
                   target="_blank">
                    {"Big O Notation"}
                </a>
                {" is a mathematical notation that describes the behavior of a value (most commonly
                denoted as 'n') as it reaches infinity. That's how you'd see it explained in most
                places."}
                <br />
                <br />
                {"The mathematical explaination aside, there's a nuance in the programming usage of
                the big O notation. It can be used in both "}
                <a href={"https://en.wikipedia.org/wiki/Time_complexity"}
                   target="_blank">
                {"Time Complexity"}
                </a>
                {" and "}
                <a href={"https://en.wikipedia.org/wiki/Space_complexity"}
                   target="_blank">
                {"Space Complexity"}
                </a>
                {". More on those in "}
                <a href={BLOG_LINK.to_owned() + link + dictionary}
                   target="_blank">
                {"the next part of this blog"}
                </a>
                {"."}
            </p>

            <h2 id={dictionary}>
                {dictionary}
            </h2>

            <p>
                <a href={"https://en.wikipedia.org/wiki/Time_complexity"}
                   target="_blank">
                {"Time Complexity"}
                </a>
                {" - It's used to describe the amount of CPU resources taken in order to finish
                evaluating the algorithm in runtime"}
            </p>

            <p>
                <a href={"https://en.wikipedia.org/wiki/Space_complexity"}
                   target="_blank">
                {"Space Complexity"}
                </a>
                {" - It's used to describe the amount of memory resources taken in order to finish
                evaluating the whole algorithm."}
            </p>

            <p>
                <a href={"https://en.wikipedia.org/wiki/Compile_time"}
                   target="_blank">
                {"Compile Time"}
                </a>
                {" - This isn't measured with Big O notation anywhere. The compile time is the time
                where your compiler to translates your witten code into bytecode/machine code
                depending if you use an interpreted language or a compiled one."}
                <br />
                <br />
                <i>
                    {"A good thing about the fact that compile time evaluations aren't accounted
                    for in sites like "}
                    <a href={"https://https://leetcode.com/"}
                    target="_blank">
                    {"LeetCode"}
                    </a>
                    {" is that you can do compile time optimizations (mainly in languages like C and
                    C++) in order to do compile time evaluation via features like "}
                    <a href={"https://en.cppreference.com/w/cpp/language/consteval"}
                    target="_blank">
                    {"Constant Evaluations"}
                    </a>
                    {" and "}
                    <a href={"https://en.cppreference.com/w/cpp/language/constexpr"}
                    target="_blank">
                    {"Constant Expressions"}
                    </a>
                    {"."}
                </i>
            </p>

            <p>
                <a href={"https://en.wikipedia.org/wiki/Runtime_(program_lifecycle_phase)"}
                   target="_blank">
                {"Runtime"}
                </a>
                {" - This is the time when the program is being executed after compiling it. "}
                <a href={"https://https://leetcode.com/"}
                target="_blank">
                {"LeetCode's"}
                </a>
                {" Time Complexity performance measurement is based on this exact metric."}
            </p>

            <h2 id={analysis}>
                {analysis}
            </h2>
            <p>
                {"It's very likely that you've seen this Big O notation graph: "}
            </p>
            <img src="/public/blog/big-o/big-o-chart.png" />
            <p>
                {"Let's go over these stuff really quickly"}
            </p>
            <ul>
                <li>
                    <img src="/public/blog/big-o/O-1.svg"
                         class="tex-svg"
                         style={format!("width: calc({} - 10px);", TEX_ITEM_WIDTH)}
                         alt="O of 1" />
                    <br />
                    {"Also referred to as constant complexity."}
                    <br />
                    {"Some of the programming data structures that match this complexity notation
                    are: pushing an item at the front of a vector; looking up an item in a hashed
                    set or a hashed map (dictionary); getting the first element of a singly linked
                    list; getting the last element of a vector."}
                    <br />
                    {"Not a lot of algorithms are done in this complexity"}
                </li>
                <li>
                    <img src="/public/blog/big-o/O-log-n.svg"
                         class="tex-svg"
                         style={format!("width: calc({} + 30px);", TEX_ITEM_WIDTH)}
                         alt="O of log of n" />
                    <br />
                    {""}
                </li>
                <li>
                    <img src="/public/blog/big-o/O-n-log-n.svg"
                         class="tex-svg"
                         style={format!("width: calc({} + 50px);", TEX_ITEM_WIDTH)}
                         alt="O of n times log of n" />
                    <br />
                    {""}
                </li>
                <li>
                    <img src="/public/blog/big-o/O-n-squared.svg"
                         class="tex-svg"
                         alt="O of n squared" />
                    <br />
                    {""}
                </li>
                <li>
                    <img src="/public/blog/big-o/O-2-to-the-power-of-n.svg"
                         class="tex-svg"
                         alt="O of 2 to the power of n" />
                    <br />
                    {""}
                </li>
                <li>
                    <img src="/public/blog/big-o/O-n-factorial.svg"
                         class="tex-svg"
                         alt="O of n factorial" />
                    <br />
                    {""}
                </li>
            </ul>
        </>
    })
}
