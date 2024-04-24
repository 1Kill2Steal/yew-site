use super::*;
use crate::utils::{set_iframe_gist, set_youtube_iframe};

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
const DETAILS_ITEM_GLYPH: &'static str = "nerd-font-glyph nf-md-subdirectory_arrow_right";
const BLOG_DETAILS_ITEM: &'static str = "blog-details-item";

fn blog_metadata(
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
        <div class={METADATA}
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
    let optimizing_for_memory_vs_space = "Optimizing_for_memory_vs_optimizing_for_space";
    let the_misinformation_about_big_o_complexity = "The_misinformation_about_big_O_notation";
    let not_only_big_o = "Not_only_big_O";
    let summary = "Summary";
    let contents = html! {
        <>
            <div class={BLOG_DETAILS_ITEM}>
                <a href={ID_REF.to_owned() + introduction}
                class={DETAILS_ITEM_GLYPH}
                >
                    {introduction}
                </a>
            </div>
            <div class={BLOG_DETAILS_ITEM}>
                <a href={ID_REF.to_owned() + dictionary}
                class={DETAILS_ITEM_GLYPH}
                >
                    {dictionary}
                </a>
            </div>
            <div class={BLOG_DETAILS_ITEM}>
                <a href={ID_REF.to_owned() + analysis}
                class={DETAILS_ITEM_GLYPH}
                >
                    {analysis}
                </a>
            </div>
            <div class={BLOG_DETAILS_ITEM}>
                <a href={ID_REF.to_owned() + optimizing_for_memory_vs_space}
                class={DETAILS_ITEM_GLYPH}
                >
                    {optimizing_for_memory_vs_space.replace('_', " ")}
                </a>
            </div>
            <div class={BLOG_DETAILS_ITEM}>
                <a href={ID_REF.to_owned() + the_misinformation_about_big_o_complexity}
                class={DETAILS_ITEM_GLYPH}
                >
                    {the_misinformation_about_big_o_complexity.replace('_', " ")}
                </a>
            </div>
            <div class={BLOG_DETAILS_ITEM}>
                <a href={ID_REF.to_owned() + not_only_big_o}
                class={DETAILS_ITEM_GLYPH}
                >
                    {not_only_big_o.replace('_', " ")}
                </a>
            </div>
            <div class={BLOG_DETAILS_ITEM}>
                <a href={ID_REF.to_owned() + summary}
                class={DETAILS_ITEM_GLYPH}
                >
                    {summary.replace('_', " ")}
                </a>
            </div>
        </>
    };
    blog_showcase_wrap(html! {
        <>
            <h1>{ "Understanding Big O Notation" }</h1>
            {blog_metadata(
                clipboard,
                "24/04/2024",
                "≈10 mins",
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
                   target="_blank"
                >
                    {"Big O Notation"}
                </a>
                {" is a mathematical notation that describes the behavior of a value (most commonly
                denoted as "}<b>{"n"}</b>{") as it reaches infinity. That's how you'd see it
                explained in most places. The mathematical explanation aside, there's a nuance in
                the programming usage of the big O notation. It can be used in both "}
                <a href={"https://en.wikipedia.org/wiki/Time_complexity"}
                   target="_blank"
                >
                    {"Time Complexity"}
                </a>
                {" and "}
                <a href={"https://en.wikipedia.org/wiki/Space_complexity"}
                   target="_blank"
                >
                    {"Space Complexity"}
                </a>
                {"."}
            </p>

            <h2 id={dictionary}>
                {dictionary}
            </h2>

            <p>
                <a href={"https://en.wikipedia.org/wiki/Time_complexity"}
                   target="_blank"
                >
                    {"Time Complexity"}
                </a>
                {" - It's used to describe the amount of CPU resources needed in order to finish
                evaluating the algorithm in runtime"}
            </p>

            <p>
                <a href={"https://en.wikipedia.org/wiki/Space_complexity"}
                   target="_blank"
                >
                    {"Space Complexity"}
                </a>
                {" - It's used to describe the amount of memory resources needed in order to finish
                evaluating the whole algorithm."}
            </p>

            <p>
                <a href={"https://en.wikipedia.org/wiki/Compile_time"}
                   target="_blank"
                >
                {"Compile Time"}
                </a>
                {" - This isn't measured with Big O notation. The compile time is the time where
                your interpreter/compiler to translates your written code into byte-code/machine code
                respectively."}
            </p>
            <p>
                <i>
                    {"A good thing about the fact that compile time evaluations aren't accounted
                    for in sites like "}
                    <a href={"https://https://leetcode.com/"}
                       target="_blank"
                    >
                        {"LeetCode"}
                    </a>
                    {" is that you can do compile time optimizations (mainly in languages like C
                    and C++) in order to do compile time evaluation via features like "}
                    <a href={"https://en.cppreference.com/w/cpp/language/consteval"}
                       target="_blank"
                    >
                        {"Constant Evaluations"}
                    </a>
                    {" and "}
                    <a href={"https://en.cppreference.com/w/cpp/language/constexpr"}
                       target="_blank"
                    >
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
                    <br />

                    {"Also referred to as constant complexity."}

                    <br />
                    <br />

                    {"Some of the programming data structures that match this complexity
                    notation are: pushing an item at the front of a vector; looking up an item
                    in a hashed set or a hashed map (dictionary); getting the first element of
                    a singly linked list; getting the last element of a vector."}

                    <br />
                    <br />

                    {"Not a lot of algorithms are done in this complexity. Usually it's a
                    simple method on a specific data structure."}
                </li>
                <li>
                    <img src="/public/blog/big-o/O-log-n.svg"
                         class="tex-svg"
                         style={format!("width: calc({} + 30px);", TEX_ITEM_WIDTH)}
                         alt="O of log of n" />

                    <br />
                    <br />

                    {"This is a somewhat common performance region in LeetCode solutions, It's
                    primary use is in "}
                    <a href="https://en.wikipedia.org/wiki/Binary_search_algorithm"
                       target="_blank"
                    >
                    {"Binary Search"}
                    </a>
                    {" (Recommended Leets: "}
                    <a href="https://leetcode.com/problems/palindrome-number/"
                       target="_blank"
                    >
                    {"Palindrome Number"}
                    </a>
                    {" and "}
                    <a href="https://leetcode.com/problems/search-insert-position/description/"
                       target="_blank"
                    >
                        {"Search Insert Position"}
                    </a>
                    {") This algorithm is common for interview questions and practical to know (and
                    it's also joked that every university student has implemented it)."}
                </li>
                <li>
                    <img src="/public/blog/big-o/O-n-log-n.svg"
                         class="tex-svg"
                         style={format!("width: calc({} + 50px);", TEX_ITEM_WIDTH)}
                         alt="O of n times log of n" />

                    <br />
                    <br />

                    {"Usually this is where sorting algorithms fall into, ones like "}
                    <a href="https://en.wikipedia.org/wiki/Quicksort"
                       target="_blank"
                    >
                        {"Quicksort"}
                    </a>
                    {" ("}
                    <i>
                        {"average case... depending on your pivot point... please don't use a pivot
                        point at the end"}
                    </i>
                    {"), "}
                    <a href="https://en.wikipedia.org/wiki/Heapsort"
                       target="_blank"
                    >
                        {"Heapsort"}
                    </a>
                        {" (in every case) and "}
                    <a href="https://en.wikipedia.org/wiki/Merge_sort"
                       target="_blank"
                    >
                        {"Merge sort"}
                    </a>
                        {" (in most cases)."}
                    <br />
                    <br />
                    <i>
                        {"This is for time complexity. The space complexities for Quicksort and
                        Heapsort are "}<b>{"O(n log n)"}</b>{" best case and "}<b>{"O(n)"}</b>{"
                        worst case and for Merge sort it's "}<b>{"O(n)"}</b>{"."}
                    </i>
                </li>
                <li>
                    <img src="/public/blog/big-o/O-n-squared.svg"
                         class="tex-svg"
                         alt="O of n squared" />

                    <br />
                    <br />

                    {"A common way to get to this point is doing 2 iterations on your array. (Did I
                    mention how 90% of LeetCode is arrays? Yeah, they're really useful. Anyway).
                    Some problems have "}<b>{"O(n"}<sup>{"2"}</sup>{")"}</b>{" as the best solutions.
                    Those problems are usually quite rare (At least from my little experience). A
                    more interesting way is matrices. Those big two-dimensional arrays you see in
                    University math classes. They can be coded into a computer too. The syntax is
                    usually arr[i][j] but that's not very important right now. The more important
                    part is that these matrices are traversed in
                    "}<b>{"O(n"}<sup>{"2"}</sup>{")"}</b>{" time complexity. Also, "}
                    <a href="https://en.wikipedia.org/wiki/Bubble_sort"
                       target="_blank"
                    >
                        {"Bubble sort"}
                    </a>
                    {" is a "}<b>{"O(n"}<sup>{"2"}</sup>{")"}</b>{" algorithm. It's very
                    impractical but it's the easiest sorting algorithm to understand so you should
                    just know it if you've been getting into the more complex ones like Quicksort,
                    Heapsort and Mergesort."}
                </li>
                <li>
                    <img src="/public/blog/big-o/O-2-to-the-power-of-n.svg"
                         class="tex-svg"
                         alt="O of 2 to the power of n" />

                    <br />
                    <br />

                    {set_iframe_gist(
                        "https://gist.github.com/1Kill2Steal/ccfd097bc8fa49f0aba87e5035ab75a2",
                        None
                    )}
                    {"Chances are that you've seen this notorious Fibonacci sequence solution. This
                    is a bad solution in practice though. If you refer to the image at the start of
                    this section. It's in the second slowest section. The solutions here (and at
                    "}<b>{"O(n!)"}</b>{") consist of brute force algorithms. They only work on
                    small data sets and have an exponential growth in their resource consumption."}
                </li>
                <li>
                    <img src="/public/blog/big-o/O-n-factorial.svg"
                         class="tex-svg"
                         alt="O of n factorial" />

                    <br />
                    <br />

                    {"Similarly to "}<b>{"O(2"}<sup>{"n"}</sup>{")"}</b>{" it's useful in brute
                    force algorithms and actually evaluating "}<b>{"n!"}</b>{" (n factorial)."}
                </li>
            </ul>

            <h2 id={optimizing_for_memory_vs_space}>
                {optimizing_for_memory_vs_space.replace('_', " ")}
            </h2>
            <p>
                {"This depends on a lot of factors. Although this will be the example use case: "}
            </p>
            <p>
                {"Finding prime numbers (credit to "}
                <a href="https://www.youtube.com/@b001"
                    target="_blank"
                >
                    {"b001"}
                </a>
                {" on YouTube):"}
            </p>
            {set_youtube_iframe(html!{
                <iframe
                    height="100%"
                    width="100%"
                    src="https://www.youtube-nocookie.com/embed/fwxjMKBMR7s"
                    title="YouTube video player"
                    frameborder="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                    referrerpolicy="strict-origin-when-cross-origin"
                    allowfullscreen=true>
                </iframe>
            })}
            <p>
                {"Let's break down the primary three ways of doing it:"}
            </p>
            <ol>
                <li>
                    {"Trial Division"}
                    <p>
                        {"The most efficient memory wise. It's the slowest in practice though
                        because it requires you to first: evaluate the square root of every number,
                        second: do modulo division for all the numbers that are less than the
                        square root of that number. How does that even work? This image from the
                        video will help understand just that:"}
                    </p>
                    <img src="/public/blog/big-o/trial-division.png"
                         style={"width: 100%;"}
                         alt="A representation of how the multiples after square root of 24 are
                         mirrored from the ones before it." />
                    <p>
                        {"Interesting right? They're mirrored. That's always the case. The
                        explanation is rather simple: "}<b>{"3"}<sup>{"2"}</sup></b>{" is 9
                        obviously, so whatever is a divisor of 3 is also a divisor of 9. If this
                        method is applied to an actual prime number here's how it'd look like: "}
                    </p>
                    <p>
                        {"Let's say 11 is the example. The square root of it is between 3 and 4
                        (closer to 3) and 4 isn't a prime number either. So all that's needed to
                        get checked is whether 11 is divisible by 2 or 3 (because 4, 6, 8, 10 are
                        already divisible by 2 and 6 and 9 are already divisible by 3 so the math
                        for them would be excessive). Or if you prefer the mathematical
                        explanation: "}
                    </p>
                    <p>
                        <i>
                            {"if n is divisible by some number p, then n = p × q and if q were smaller
                            than p, n would have been detected earlier as being divisible by q or by a
                            prime factor of q."}
                        </i>
                        {" ("}
                        <a href="https://en.wikipedia.org/wiki/Trial_division#Method"
                            target="_blank"
                        >
                            {"Wikipedia"}
                        </a>
                        {")"}
                    </p>
                    <p>
                        {"The time and space complexities for this algorithm are tricky to explain
                        since they revolve around using square roots, I recommend checking "}
                        <a href="https://en.wikipedia.org/wiki/Trial_division#Speed"
                            target="_blank"
                        >
                            {"this Wikipedia section"}
                        </a>
                        {" if you're interested in the exact details. The only thing that's for
                        sure the case is that Trial division takes "}<b><u>{"less"}</u></b>{" space
                        than both Sieve of Eratosthenes and Dijkstra's approach but it's the
                        slowest one in time complexity."}
                    </p>
                </li>
                <li>
                    <a href="https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes"
                       target="_blank"
                    >
                        {"Sieve of Eratosthenes"}
                    </a>
                    <p>
                        {"This one is the quickest in time complexity of O(n log log n) but it has
                        O(n) memory complexity. The application is pretty simple: get all the
                        numbers from 2 included to n included in an array of boolean values (true
                        by default), then start at 2, go through all multiples of 2 and turn them
                        to false, then do 3 and go through all multiples of 3 (ignoring all already
                        false multiples) and turn them to false. Do that for every number up to n
                        included."}
                    </p>
                    <p>
                        {"Despite being O(n) space complexity it also manages to tell you all the
                        prime numbers up to n so it's incredibly efficient in bulk prime number
                        evaluation. In fact it's hands down the best algorithm for that."}
                    </p>
                </li>
                <li>
                    {"Dijkstra's Approach"}
                    <p>
                        {"This one is a bit trickier to explain (you can see this visually at 10:26
                        from the video) but here's how it goes: "}
                    </p>
                    <p>
                        {"Unlike the first 2 algorithms this one uses a \"pool\" and the usual list
                        of prime numbers (where the initial numbers in the pool are also in the
                        prime list). We skip 1 as usual. At 2 we get 2 and it's multiple (4) and
                        add them to the pool as a "}
                        <a href="https://en.wikipedia.org/wiki/Tuple"
                        target="_blank"
                        >
                            {"tuple"}
                        </a>
                        {", then we go to 3, we check the smallest multiple in the pool (4) and if
                        it's larger than our current number (3) we add 3 and its multiple 9 as a
                        tuple of (3, 9) to the pool. The next number is 4, we check the smallest
                        multiple in the pool again, if they're equal then "}
                        <b>
                            {"increment the smallest multiple by its associated number"}
                        </b>
                        {". This means that the (2, 4) pair will become (2, 6). Then we go to 5,
                        it's smaller than the smallest multiple so we add it with its multiple in
                        the pool."}
                        <br />
                        {"The next one is 6 -> we change the (2, 6) pair to (2, 8)."}
                        <br />
                        {"7 -> it's smaller than the smallest so we add it to the pool with its multiple."}
                        <br />
                        {"8 -> change (2, 8) to (2, 10)."}
                        <br />
                        {"9 -> change (3, 9) to (3, 12). 10 -> change (2, 10) to (2, 12)"}
                        <br />
                        {"11 -> Add it to the pool with its multiple"}
                    </p>
                </li>
            </ol>
            <h4>
                {"Conclusion"}
            </h4>
            <p>
                {"If you only want memory efficiency: do Trial Division."}
                <br />
                {"If you only want speed: use the Sieve of Eratosthenes."}
                <br />
                {"If you want a balance between the two: try out Dijkstra's approach."}
            </p>
            <p>
                {"As stated in the start of this section. There are a lot of factors to determine
                when going for memory over speed or vice versa. For example in game development
                memory efficiency in saving resources for your models is also a speed improvement.
                The most important factor in determining whether you want to optimize for memory or
                for speed is the ability to determine what you need more."}
            </p>

            <h2 id={the_misinformation_about_big_o_complexity}>
                {the_misinformation_about_big_o_complexity.replace('_', " ")}
            </h2>
            <p>
                {"Big O notation ISN'T the all-be-it in determining whether or not algorithm A is
                more efficient than algorithm B. There's more to it. The reason is this: Big O
                notation, as described in the start, is the behavior of a value (n) as it reaches
                infinity (or in mathematical notation that'd be: n → ∞). This specific notation
                eliminates all constant values in its determination. This means that O(2n) is
                portrayed as O(n) (linear), just in the same way that O(1,000n) is portrayed as
                O(n), so is O(1,000,000n) and so on. The reason for this is because big O notation
                was meant to only account for the worst case scenario of an algorithm. That worst
                case scenario being the upper limit of n as it reaches infinity, and because it's
                only looking at that, it removes all other constants because eventually n will be
                greater than them. But is this true in practice? In most cases it's not, some
                of the times you even know your exact data set as well! That means that you can
                optimize an algorithm based exactly on that data set."}
            </p>

            <h2 id={not_only_big_o}>
                {not_only_big_o.replace('_', " ")}
            </h2>

            // Big O
            <p>
                {"Big O notation describes an algorithm based on its worst case scenario as n grows to infinity. This means it's denoted as this: "}
            </p>
            <p align="center">
                <i>
                {"0 <= f(n) <= Cg(n) for all n >= n0"}
                </i>
            </p>
            <p>
                {"Where:"}
            </p>
            <ul>
                <li>
                    {"Everything is greater than or equal to 0"}
                </li>
                <li>
                    {"The running time function that operates on the data set (n) is greater than
                    or equal to 0"}
                </li>
                <li>
                    {"The Constant growth of n is greater than or equal to the f(n)"}
                </li>
                <li>
                    {"The notation applies for every value of n which is greater than
                    "}<b>{"n0"}</b>{" ("}<b>{"n0"}</b>{" being a positive constant similarly to
                    "}<b>{"C"}</b>{")."}
                </li>
            </ul>
            <img src="/public/blog/big-o/big-o-example.png" />
            <p align="center">
                <i>
                    {"Credit for all the images in this section"}
                    <br />
                    <a href="https://www.geeksforgeeks.org/difference-between-big-oh-big-omega-and-big-theta/"
                       target="_blank"
                    >
                        {"Geeks for Geeks"}
                    </a>
                </i>
            </p>
            <p>
                {"In this worst case we see how "}<b>{"n0"}</b>{" is determined (it takes the
                highest intersecting point of "}<b>{"Cg(n)"}</b>{" and "}<b>{"f(n)"}</b>{"). This
                eliminates all the quicker to process data sets and only works with the "}
                <a href="https://en.wikipedia.org/wiki/Upper_bound"
                   target="_blank"
                >
                    {"upper bound"}
                </a>
                {" of "}<b>{"n"}</b>{"."}
            </p>

            <hr />

            // Big Omega
            <p>
                {"Big Omega (Ω) notation describes an algorithm based on its best case scenario
                given the lower bound of n. This means it's denoted as this: "}
            </p>
            <p align="center">
                <i>
                    {"0 <= Cg(n) <= f(n) for all n >= n0"}
                </i>
            </p>
            <p>
                {"Where:"}
            </p>
            <ul>
                <li>
                    {"Everything is greater than or equal to 0"}
                </li>
                <li>
                    {"The Constant growth of n is greater than or equal to 0"}
                </li>
                <li>
                    {"The running time function that operates on the data set (n) is greater than
                    or equal to Cg(n)"}
                </li>
                <li>
                    {"The notation applies for every value of n which is greater than
                    "}<b>{"n0"}</b>{" ("}<b>{"n0"}</b>{" being a positive constant similarly to
                    "}<b>{"C"}</b>{")."}
                </li>
            </ul>
            <img src="/public/blog/big-o/big-omega-example.png" />
            <p>
                {"Huh? That's quite similar right? Yeah, the constant is just lower here, that's
                their difference. It can also be equal which makes it compatible with all 3
                notation types (oh no that was a spoiler)."}
            </p>

            <hr />

            // Big Theta
            <p>
                {"Big Theta (Θ) notation is used to describe the tightest bound between the worst
                case and the best case scenarios."}
            </p>
            <p align="center">
                <i>
                    {"0 <= f(n) <= C1g(n) for n >= n0"}
                    <br />
                    {"0 <= C2g(n) <= f(n) for n >= n0"}
                </i>
            </p>
            <p>
                {"This time we have 2 constants. The 2 equations are the same from the Big O and
                Big Omega (Ω) notations respectively."}
                <br />
                {"Ultimately we get a merged equation which looks like this:"}
            </p>
            <p align="center">
                <i>
                    {"0 <= C2g(n) <= f(n) <= C1g(n) for n >= n0"}
                </i>
            </p>
            <p>
                {"Where:"}
            </p>
            <ul>
                <li>
                    {"Everything is greater than or equal to 0"}
                </li>
                <li>
                    {"The Second Constant growth of n is greater than or equal to 0"}
                </li>
                <li>
                    {"The running time function that operates on the data set ("}<b>{"n"}</b>{") is
                    greater than or equal to "}<b>{"C2g(n)"}</b>{"."}
                </li>
                <li>
                    {"The First Constant growth of n is greater than or equal to
                    "}<b>{"f(n)"}</b>{"."}
                </li>
                <li>
                    {"The notation applies for every value of n which is greater than
                    "}<b>{"n0"}</b>{" ("}<b>{"n0"}</b>{" being a positive constant similarly to
                    "}<b>{"C"}</b>{")."}
                </li>
            </ul>
            <img src="/public/blog/big-o/big-theta-example.png" />
            <h4>
                {"Conclusion"}
            </h4>
            <p>
                {"If we simplify it:"}
            </p>
            <p>
                {"Big O is like >="}
            </p>
            <p>
                {"Big Ω is like <="}
            </p>
            <p>
                {"Big Θ is like =="}
            </p>
            <p>
                {"It's not too important to delve so much on the technical details rather than just
                knowing how the bounds are affected in all of those 3 algorithms. Big O being worst
                case, Big Ω being best case and Big Θ tightly bounded (evenly distributed) cases."}
            </p>

            <h2 id={summary}>
                {summary}
            </h2>
            <p>
                {"The big O notation is a mathematical way to define the worst case scenario
                performance of an algorithm as its data set reaches infinity. This means it
                evaluates its performance based on the upper bound of the data set. The performance
                measured is separated in 2 types: runtime performance (how much your CPU needs to
                operate on the data set) and memory performance (how much runtime memory your
                algorithm takes based on your data set). Determining which part of your program you
                want to optimize depends heavily on your use case. You have to evaluate which one
                is the better one to optimize for. Also, Big O notation is for the worst case
                scenarios, there's also Big Ω (Omega) notation for the best case scenarios (where
                your constant that operates on the data set is less than or equal to your functions
                input) and a Big Θ (Theta) notation where your growth rate has to be even
                throughout the growth of your data set. This means that everything that can be
                described in Big Theta can also be described in Big Omega and Big O notations."}
            </p>
        </>
    })
}
