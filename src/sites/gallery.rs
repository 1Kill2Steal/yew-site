use super::*;

use gloo_net::http::Request;

use crate::templates::footer::Footer;
use crate::templates::nav::Nav;

// You can change some file path and image name pattern stuff here.
use crate::data::gallery::*;

#[function_component(Gallery)]
pub fn gallery() -> Html {
    // Who would've thought I'd use someone else's GitHub repo code for reference...
    // I'm sorry and thank you at the same time
    // https://github.com/LelouchFR/windows-terminal-theme-generator/blob/61262073be3af7c39468c18b9cf8835683e00495/src/home_page.rs#L50-L63
    #[allow(clippy::redundant_closure)] // The closure actually isn't redundant.
    let data: UseStateHandle<JsonFolderSizesLayout> =
        use_state(|| JsonFolderSizesLayout::default());
    {
        let data = data.clone();
        use_effect_with((), move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data: JsonFolderSizesLayout = Request::get(JSON_FOLDER_SIZES)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                data.set(fetched_data);
            });

            || ()
        });
    }
    // You can use either or (doesn't matter) Made sure the count of compressed items matches the
    // count of the uncompressed items.
    let compressed_pics_folder_size = data.clone().compressed_count();

    // Page values.
    let first_quarter = PAGE_SIZE / 4;
    let second_quarter = first_quarter + first_quarter;
    let third_quarter = second_quarter + first_quarter;
    let total_pages = if compressed_pics_folder_size / PAGE_SIZE % PAGE_SIZE == 0 {
        compressed_pics_folder_size / PAGE_SIZE
    } else {
        compressed_pics_folder_size / PAGE_SIZE + 1
    };

    // Hooks
    let current_page = use_state(|| 1);
    let move_state = |distance: i32| {
        let counter = current_page.clone();
        let target = *counter + distance;

        let (first_page_jump, last_page_jump, overflow, underflow): (bool, bool, bool, bool) = (
            distance == 0,
            distance >= total_pages || target == 0,
            target > total_pages,
            target < 0,
        );

        if first_page_jump {
            return Callback::from(move |_| counter.set(1));
        }
        if last_page_jump {
            return Callback::from(move |_| counter.set(total_pages));
        }
        if overflow {
            return Callback::from(move |_| counter.set(target - total_pages));
        }
        if underflow {
            return Callback::from(move |_| counter.set(total_pages - target.abs()));
        }

        Callback::from(move |_| counter.set(target))
    };

    let selected_img_id = use_state(|| 0);
    let handle_img_click = |image_id: i32| {
        let selected_img_id = selected_img_id.clone();
        Callback::from(move |_| selected_img_id.set(image_id))
    };

    // Hook dependant variables
    let (prev_img, next_img): (i32, i32) = {
        let selected = *selected_img_id.clone();
        let page = *current_page.clone();
        let page_end = page * PAGE_SIZE + 1;
        let page_start = (page_end) - PAGE_SIZE - 1;

        if selected - 1 <= page_start {
            (selected, selected + 1)
        } else if selected + 1 >= page_end || selected >= compressed_pics_folder_size {
            (selected - 1, selected)
        } else {
            (selected - 1, selected + 1)
        }
    };
    let (set_close_button, set_next_button, set_prev_button) = {
        if *selected_img_id <= 0 {
            (
                "hidden-image-close-button",
                "hidden-next-image-button",
                "hidden-prev-image-button",
            )
        } else {
            (
                "image-close-button",
                "next-image-button",
                "prev-image-button",
            )
        }
    };
    let pagination = || {
        html! {
        <div class="pagination">
            <span>{ format!("Page {} of {}", *current_page, total_pages) }</span>
            <button onclick={move_state(0)}>{ "Start" }</button>
            <button onclick={move_state(-3)}>{ "-3" }</button>
            <button onclick={move_state(-2)}>{ "-2" }</button>
            <button onclick={move_state(-1)}>{ "-1" }</button>
            <button onclick={move_state(1)}>{ "+1" }</button>
            <button onclick={move_state(2)}>{ "+2" }</button>
            <button onclick={move_state(3)}>{ "+3" }</button>
            // I don't know why this works when the total_pages is inverted...
            <button onclick={move_state(total_pages)}>{ "End" }</button>
        </div>
        }
    };

    // Outer closure: iterates over all the items in the target page (defined by hooks)
    let set_images = |start: i32,
                      end: i32,
                      page: yew::UseStateHandle<i32>,
                      key: Option<String>,
                      class_name: Option<String>,
                      id_name: Option<String>| {
        let page = (*page - 1) * PAGE_SIZE;
        let page_start = start + page;
        let page_end = (end - 1) + page;

        // The inner closure gets ran for every image in the start-end range of the outer closure.
        let set_dom = |id: i32, img_name: String| {
            let current_img =
                format!("{PICS_COMPRESSED_FOLDER_NAME}{img_name}{COMPRESSED_IMAGE_EXTENSION}");
            let current_uncompressed_img =
                format!("{PICS_UNCOMPRESSED_FOLDER_NAME}{img_name}{UNCOMPRESSED_IMAGE_EXTENSION}");
            let selected_img_class = || -> &'static str {
                if *selected_img_id == id {
                    FULLSCREEN_OVERLAY_CLASS_NAME
                } else {
                    ""
                }
            };

            let (wrapper, some_fullscreen_img) = if !selected_img_class().is_empty() {
                ("", true)
            } else {
                ("img-wrapper", false)
            };

            let download_text = || "Download";

            let display_img =
                |class: Option<&'static str>, img_class: Option<&'static str>, src: String| {
                    html! {
                        <div key={img_name.clone()}
                             class={class.unwrap_or("")}
                        >
                            <div class={img_class.unwrap_or("")}
                                 onclick={handle_img_click(id)}
                            >
                                <img src={src} />
                            </div>
                            // Notice that it's the uncompressed one
                            <a href={current_uncompressed_img.clone()}
                                download={img_name.clone()}
                                alt={img_name.replace('_', " ")}
                            >
                                { download_text() }
                            </a>
                        </div>
                    }
                };
            html! {
                <>
                    if some_fullscreen_img {
                        {
                            display_img(
                                Some(FULLSCREEN_OVERLAY_CLASS_NAME),
                                Some(FULLSCREEN_IMG_CLASS_NAME),
                                current_uncompressed_img.clone()
                            )
                        }
                    }
                    // The normal image gets displayed nomatter what
                    {display_img(None, Some(wrapper), current_img.clone())}
                </>
            }
        };
        html! {
            <div key={key.unwrap_or(format!("{start} to {end}"))} class={class_name} id={id_name}>
            {((page_start)..=(page_end)).map(|img_num| {
                if img_num > compressed_pics_folder_size {
                    html!{}
                } else {
                    set_dom(img_num, format!("{IMAGE_NAME_PATTERN}{img_num}"))
                }
            }).collect::<Html>()}
            </div>
        }
    };

    // Final HTML
    html! {
        <root>
            <div class={CONTENT}>
                <Nav />
                <div class={SITE_CONTENT}>
                    <h1>
                        { "Hu Tao Gallery" }
                        <br />
                        { "(VERY IMPORTANT!)" }
                    </h1>
                    <button class={classes!(set_close_button)}
                            onclick={handle_img_click(0)}
                    >
                        {"X"}
                    </button>
                    <button class={classes!(set_prev_button)}
                            onclick={handle_img_click(prev_img)}
                    >
                        {"<"}
                    </button>
                    <button class={classes!(set_next_button)}
                            onclick={handle_img_click(next_img)}
                    >
                        {">"}
                    </button>
                    {pagination()}
                    <section class="hu-tao-gallery">
                        {set_images(1,
                            first_quarter,
                            current_page.clone(),
                            Some(String::from("column_1")),
                            Some(String::from("column")),
                            None
                        )}

                        {set_images(
                            first_quarter,
                            second_quarter,
                            current_page.clone(),
                            Some(String::from("column_2")),
                            Some(String::from("column")),
                            None
                        )}

                        {set_images(
                            second_quarter,
                            third_quarter,
                            current_page.clone(),
                            Some(String::from("column_3")),
                            Some(String::from("column")),
                            None
                        )}

                        {set_images(
                            third_quarter,
                            PAGE_SIZE+1,
                            current_page.clone(),
                            Some(String::from("column_4")),
                            Some(String::from("column")),
                            None
                        )}
                    </section>
                    {pagination()}
                </div>
            </div>
            <Footer />
        </root>
    }
}
