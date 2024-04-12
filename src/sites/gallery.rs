use super::*;

use crate::data::{
    COMPRESSED_IMAGE_EXTENSION, COMPRESSED_PICS_FOLDER_SIZE, IMAGE_NAME_PATTERN,
    PICS_COMPRESSED_FOLDER_NAME, PICS_UNCOMPRESSED_FOLDER_NAME, UNCOMPRESSED_IMAGE_EXTENSION,
    UNCOMPRESSED_PICS_FOLDER_SIZE,
};
use crate::templates::footer::Footer;
use crate::templates::nav::Nav;

#[function_component(Gallery)]
pub fn gallery() -> Html {
    // This needs to be systematic for the image displaying to work.
    // The image names need to have the following pattern:
    // {IMAGE_NAME_PATTERN}{image_number}{COMPRESSED_IMAGE_EXTENSION}
    // And the extension also needs to be systematic.
    // To set the proper data for all of that please head to the data crate.

    let page_size = 20;
    let current_page = use_state(|| 1);

    let total_pages = if COMPRESSED_PICS_FOLDER_SIZE as i32 / page_size % page_size == 0 {
        COMPRESSED_PICS_FOLDER_SIZE as i32 / page_size
    } else {
        COMPRESSED_PICS_FOLDER_SIZE as i32 / page_size + 1
    };

    let first_quarter = page_size / 4;
    let second_quarter = first_quarter + first_quarter;
    let third_quarter = second_quarter + first_quarter;

    let move_state = |distance: i32| {
        let counter = current_page.clone();

        // Edge cases handling
        let (negative, positive): (bool, bool) = (distance < 0, distance > 0);
        if negative && *counter + distance < 1 {
            let res = if distance.abs() >= total_pages {
                total_pages
            } else {
                total_pages - distance.abs() + 1
            };
            return Callback::from(move |_| counter.set(res));
        }
        let distance_sum = *counter + distance;
        if positive && distance_sum >= total_pages {
            let difference = distance_sum - total_pages;
            let res = if difference == 0 {
                total_pages
            } else {
                difference
            };
            return Callback::from(move |_| counter.set(res));
        }
        if distance == 0 {
            return Callback::from(move |_| counter.set(1));
        }

        Callback::from(move |_| counter.set(*counter + distance))
    };
    let selected_img_id = use_state(|| 0);
    let fullscreen_img_class_name = "fullscreen-img";
    let selected_img_class = |target_id: i32| {
        if *selected_img_id == target_id {
            fullscreen_img_class_name
        } else {
            ""
        }
    };
    let handle_img_click = |id: i32| {
        let selected_img_id = selected_img_id.clone();
        Callback::from(move |_| selected_img_id.set(id))
    };
    let (prev_img, next_img): (i32, i32) = {
        let selected = *selected_img_id.clone();
        let page = *current_page.clone();
        let page_end = page * page_size + 1;
        let page_start = (page_end) - page_size - 1;

        if selected - 1 <= page_start {
            (selected, selected + 1)
        } else if selected + 1 >= page_end || selected >= UNCOMPRESSED_PICS_FOLDER_SIZE as i32 {
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

    let set_images = |start: i32,
                      end: i32,
                      page: yew::UseStateHandle<i32>,
                      key: Option<String>,
                      class_name: Option<String>,
                      id_name: Option<String>| {
        let page = (*page - 1) * page_size;
        let page_start = start + page;
        let page_end = (end - 1) + page;

        let mut overflow = false;
        let set_dom = |id: i32, img_name: String| {
            let current_img =
                format!("{PICS_COMPRESSED_FOLDER_NAME}{img_name}{COMPRESSED_IMAGE_EXTENSION}");
            let current_uncompressed_img =
                format!("{PICS_UNCOMPRESSED_FOLDER_NAME}{img_name}{UNCOMPRESSED_IMAGE_EXTENSION}");
            let (displayed_img, wrapper) = if !selected_img_class(id).is_empty() {
                (current_uncompressed_img.to_owned(), "")
            } else {
                (current_img.to_owned(), "img-wrapper")
            };
            let base = || {
                html! {
                    <div key={img_name.clone()} class={classes!{selected_img_class(id)}}>
                        <div class={classes!{selected_img_class(id)}} onclick={handle_img_click(id)}>
                            <img src={displayed_img.clone()} />
                        </div>
                        <a href={current_uncompressed_img.clone()}
                            download={img_name.clone()}
                            alt={img_name.replace('_', " ")}>
                            { "Download" }
                        </a>
                    </div>
                }
            };
            html! {
                <>
                if !selected_img_class(id).is_empty() {
                    {base()}
                } else {
                    {html!{}}
                }
                    <div key={img_name.clone()} class={""}>
                        <div class={wrapper} onclick={handle_img_click(id)}>
                            <img src={displayed_img.clone()} />
                        </div>
                        <a href={current_uncompressed_img.clone()}
                            download={img_name.clone()}
                            alt={img_name.replace('_', " ")}>
                            { "Download" }
                        </a>
                    </div>
                </>
            }
        };
        html! {
            <div key={key.unwrap_or(format!("{start} to {end}"))} class={class_name} id={id_name}>
            {((page_start)..=(page_end)).map(|img_num| {
                if overflow || img_num > COMPRESSED_PICS_FOLDER_SIZE as i32 {
                    overflow = true;
                    html!{}
                } else {
                    set_dom(img_num, format!("{IMAGE_NAME_PATTERN}{img_num}"))
                }
            }).collect::<Html>()}
            </div>
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
            <button onclick={move_state(-total_pages)}>{ "End" }</button>
        </div>
        }
    };
    html! {
        <root>
            <div class="content">
                <Nav />
                <div class="site-content">
                    <h1>
                        { "Hu Tao Gallery" }
                        <br />
                        { "(VERY IMPORTANT!)" }
                    </h1>
                    <button class={classes!(set_close_button)}
                            onclick={handle_img_click(0)}>
                            {"X"}
                    </button>
                    <button class={classes!(set_prev_button)}
                            onclick={handle_img_click(prev_img)}>
                            {"<"}
                    </button>
                    <button class={classes!(set_next_button)}
                            onclick={handle_img_click(next_img)}>
                            {">"}
                    </button>
                    {pagination()}
                    <section class="hu-tao-gallery">
                        {
                            set_images(1,
                                first_quarter,
                                current_page.clone(),
                                Some(String::from("column_1")),
                                Some(String::from("column")),
                                None)
                        }
                        {
                            set_images(first_quarter,
                                second_quarter,
                                current_page.clone(),
                                Some(String::from("column_2")),
                                Some(String::from("column")),
                                None)
                        }
                        {
                            set_images(second_quarter,
                                third_quarter,
                                current_page.clone(),
                                Some(String::from("column_3")),
                                Some(String::from("column")),
                                None)
                        }
                        {
                            set_images(third_quarter,
                                page_size+1,
                                current_page.clone(),
                                Some(String::from("column_4")),
                                Some(String::from("column")),
                                None)
                        }
                    </section>
                    {pagination()}
                </div>
            </div>
            <Footer />
        </root>
    }
}
