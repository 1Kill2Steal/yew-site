use crate::templates::footer::Footer;
use crate::templates::nav::Nav;

use yew::functional::*;
use yew::prelude::*;

#[function_component(Gallery)]
pub fn gallery() -> Html {
    // This needs to be systematic for the image displaying to work.
    // The image names need to have the following pattern:
    // {image_name}{image_number}{image_extension}
    // And the extension also needs to be systematic.
    let image_name = String::from("Hu_Tao_");
    let image_extension = String::from(".jpg");
    let compressed_pics_dir = String::from("/hutao/pics/");
    let uncompressed_pics_dir = String::from("/hutao/pics_uncompressed/");

    let page_size = 20;
    let current_page = use_state(|| 1);

    let image_count_plus_one = 1203;
    let total_pages = image_count_plus_one / page_size + 1;

    let first_quarter = page_size / 4;
    let second_quarter = first_quarter + first_quarter;
    let third_quarter = second_quarter + first_quarter;

    let move_page = |distance: i32| {
        let counter = current_page.clone();

        // Edge cases handling
        let (negative, positive): (bool, bool) = (distance < 0, distance > 0);
        if negative && *counter + distance < 1 {
            let res = if distance.abs() == 1 || distance.abs() >= total_pages {
                total_pages
            } else {
                total_pages - distance.abs()
            };
            return Callback::from(move |_| counter.set(res));
        }
        let distance_sum = *counter + total_pages;
        if positive && distance_sum >= total_pages {
            let res = if distance_sum < total_pages { distance_sum } else { total_pages };
            return Callback::from(move |_| counter.set(res));
        }
        if distance == 0 {
            return Callback::from(move |_| counter.set(1));
        }

        Callback::from(move |_| counter.set(*counter + distance))
    };

    let set_images = |start: i32,
                      end: i32,
                      page: yew::UseStateHandle<i32>,
                      class_name: Option<String>,
                      id_name: Option<String>| {
        let page = (*page - 1) * page_size;
        let page_start = start + page;
        let page_end = (end - 1) + page;
        let mut overflow = false;
        let set_dom = |img_full_name: String| {
            let img_png_name = img_full_name.replace(".jpg", ".png");
            let no_ext = img_png_name.replace(".png", "");
            html! {
                <div tag={img_full_name.clone()}>
                    <img src={format!("{compressed_pics_dir}{img_full_name}")} />
                    <a href={format!("{uncompressed_pics_dir}{img_png_name}")}
                       download={no_ext.clone()}
                       alt={no_ext.replace('_', " ")}>
                        { "Download" }
                    </a>
                </div>
            }
        };
        html! {
            <div class={class_name} id={id_name}>
            {((page_start)..=(page_end)).into_iter().map(|img_num| {
                if img_num >= image_count_plus_one {
                    overflow = true;
                    html!{}
                } else {
                    set_dom(format!("{image_name}{img_num}{image_extension}"))
                }
            }).collect::<Html>()}
            </div>
        }
    };
    let pagination = || {
        html! {
        <div class="pagination">
            <span>{ format!("Page {} of {}", *current_page, total_pages) }</span>
            <button onclick={move_page(0)}>{ "Start" }</button>
            <button onclick={move_page(-3)}>{ "-3" }</button>
            <button onclick={move_page(-2)}>{ "-2" }</button>
            <button onclick={move_page(-1)}>{ "-1" }</button>
            <button onclick={move_page(1)}>{ "+1" }</button>
            <button onclick={move_page(2)}>{ "+2" }</button>
            <button onclick={move_page(3)}>{ "+3" }</button>
            // I don't know why this works when the total_pages is inverted...
            <button onclick={move_page(-total_pages)}>{ "End" }</button>
        </div>
        }
    };
    html! {
        <root>
            <div class="content">
                <Nav />
                <h1>
                    { "Hu Tao Gallery" }
                    <br />
                    { "(VERY IMPORTANT!)" }
                </h1>
                {pagination()}
                <section class="hu_tao_gallery">
                    {
                        set_images(1,
                            first_quarter,
                            current_page.clone(),
                            Some(String::from("column")),
                            None)
                    }
                    {
                        set_images(first_quarter,
                            second_quarter,
                            current_page.clone(),
                            Some(String::from("column")),
                            None)
                    }
                    {
                        set_images(second_quarter,
                            third_quarter,
                            current_page.clone(),
                            Some(String::from("column")),
                            None)
                    }
                    {
                        set_images(third_quarter,
                            page_size,
                            current_page.clone(),
                            Some(String::from("column")),
                            None)
                    }
                </section>
                {pagination()}
            </div>
            <Footer />
        </root>
    }
}
