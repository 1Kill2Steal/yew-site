use super::*;

#[function_component(Blog)]
pub fn blog() -> Html {
    let nav = use_navigator().unwrap();

    let callback_nav = |target: Route| {
        let nav = nav.clone();
        Callback::from(move |_| nav.clone().push(&target))
    };
    wrap_site(html! {
        <>
            <h1>{ "Blogs" }</h1>
            <div tag={"blogs_list"} class={"blogs-list"}>
                <div tag={"understanding_big_o_notation"} class={"blog-item"}>
                    <p>
                        <a href={"javascript:void(0);"} onclick={callback_nav(Route::BlogUnderstandingBigONotation)}>
                            {"Understanding Big O notation"}
                        </a>
                    </p>
                </div>
            </div>
        </>
    })
}
