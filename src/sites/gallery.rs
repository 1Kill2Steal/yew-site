#![allow(clippy::redundant_closure)]

use super::*;

use gloo_net::http::Request;

// You can change some file path and image name pattern stuff here.
use crate::data::gallery::*;

#[function_component(Gallery)]
pub fn gallery() -> Html {
    // Who would've thought I'd use someone else's GitHub repo code for reference...
    // I'm sorry and thank you at the same time
    // https://github.com/LelouchFR/windows-terminal-theme-generator/blob/61262073be3af7c39468c18b9cf8835683e00495/src/home_page.rs#L50-L63
    let request_state: UseStateHandle<bool> = use_state(|| false);
    let (file_size_data, file_name_data, artist_credits): (
        UseStateHandle<JsonFolderSizesLayout>,
        UseStateHandle<JsonImageDetailsLayout>,
        UseStateHandle<JsonArtistCredits>,
    ) = (
        use_state(|| JsonFolderSizesLayout::default()),
        use_state(|| JsonImageDetailsLayout::default()),
        use_state(|| JsonArtistCredits::default()),
    );
    if !*request_state {
        let request_state = request_state.clone();
        let file_size_data = file_size_data.clone();
        let file_name_data = file_name_data.clone();
        let artist_credits = artist_credits.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let fetched_file_size_data: JsonFolderSizesLayout = Request::get(JSON_FOLDER_SIZES)
                // .header(key, value)
                .send()
                .await
                .expect("Failed to get the response from the requested JSON")
                .json()
                .await
                .expect("Failed to fetch images from JSON");
            let fetched_file_name_data: JsonImageDetailsLayout = Request::get(JSON_IMAGE_DETAILS)
                // .header(key, value)
                .send()
                .await
                .expect("Failed to get the response from the requested JSON")
                .json()
                .await
                .expect("Failed to fetch images from JSON");

            let fetched_artist_credits: JsonArtistCredits = Request::get(JSON_ARTIST_CREDITS)
                // .header(key, value)
                .send()
                .await
                .expect("Failed to get the response from the requested JSON")
                .json()
                .await
                .expect("Failed to fetch images from JSON");

            file_size_data.set(fetched_file_size_data);
            file_name_data.set(fetched_file_name_data);
            artist_credits.set(fetched_artist_credits);
        });
        request_state.set(true);
    }
    // You can use either or (doesn't matter) Made sure the count of compressed items matches the
    // count of the uncompressed items.
    let compressed_pics_folder_size = file_size_data.clone().compressed_count();

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
            let selected_img_class: &str = {
                if *selected_img_id == id {
                    FULLSCREEN_OVERLAY_CLASS_NAME
                } else {
                    ""
                }
            };
            let current_artist = artist_credits
                .artist_credits
                .get(&(id as u32))
                // NOTE: The unwrap shouldn't ever fail if you run the build.sh script tests.
                .unwrap_or(&"".to_string())
                .clone()
                .to_string();

            let (wrapper, some_fullscreen_img) = if !selected_img_class.is_empty() {
                ("", true)
            } else {
                ("img-wrapper", false)
            };

            let download_text = || "Download";

            let img_id_html = || format!("img_id={id}");

            let display_img = |class: Option<&'static str>,
                               img_class: Option<&'static str>,
                               img_id: Option<String>,
                               src: String,
                               image_artist_box: String| {
                html! {
                    <div key={img_name.clone()}
                         class={class.unwrap_or("")}
                    >
                        <div class={img_class.unwrap_or("")}
                             id={img_id.unwrap_or(String::from(""))}
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
                        {
                            if current_artist.is_empty() {
                                html! {
                                    <a href={"javascript:void(0)"}
                                    class={image_artist_box.clone()}
                                    >
                                        {"Unknown artist"}
                                    </a>
                                }
                            } else {
                                html! {
                                    <a href={current_artist.chars().take_while(|c| *c != ' ').collect::<String>()}
                                       target="_blank"
                                       class={image_artist_box}
                                    >
                                        {&current_artist.split_whitespace().skip(1).collect::<String>()}
                                    </a>
                                }
                            }
                        }

                    </div>
                }
            };
            html! {
                <>
                    if some_fullscreen_img {
                        {
                            display_img(
                                Some(FULLSCREEN_OVERLAY_CLASS_NAME),
                                Some(FULLSCREEN_IMG_CSS_NAME),
                                Some(format!("{}&{}", FULLSCREEN_IMG_CSS_NAME, img_id_html())),
                                current_uncompressed_img.clone(),
                                IMAGE_ARTIST_BOX_NAME.to_owned()
                            )
                        }
                    }
                    // The normal image gets displayed nomatter what
                    {display_img(
                        None,
                        Some(wrapper),
                        Some(img_id_html()),
                        current_img.clone(),
                        String::from("hidden-") + IMAGE_ARTIST_BOX_NAME
                    )}
                </>
            }
        };
        html! {
            <div key={key.unwrap_or(format!("{start} to {end}"))} class={class_name} id={id_name}>
            {((page_start)..=(page_end)).map(|img_num| {
                if img_num > compressed_pics_folder_size {
                    html!{}
                } else {
                    set_dom(
                        img_num,
                        file_name_data
                            .uncompressed_dir_img_names
                            .get(&( img_num as u32 ))
                            .unwrap()
                            .replace(".png", "")
                    )
                }
            }).collect::<Html>()}
            </div>
        }
    };

    // Final HTML
    wrap_site(html! {
        <>
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
                {set_images(
                    1,
                    first_quarter+1,
                    current_page.clone(),
                    Some(String::from("column_1")),
                    Some(String::from("column")),
                    None
                )}

                {set_images(
                    first_quarter+1,
                    second_quarter+1,
                    current_page.clone(),
                    Some(String::from("column_2")),
                    Some(String::from("column")),
                    None
                )}

                {set_images(
                    second_quarter+1,
                    third_quarter+1,
                    current_page.clone(),
                    Some(String::from("column_3")),
                    Some(String::from("column")),
                    None
                )}

                {set_images(
                    third_quarter+1,
                    PAGE_SIZE+1,
                    current_page.clone(),
                    Some(String::from("column_4")),
                    Some(String::from("column")),
                    None
                )}
            </section>
            {pagination()}
        </>
    })
}
