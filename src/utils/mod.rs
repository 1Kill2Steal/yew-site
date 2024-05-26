use std::ops::Add;

use super::*;

/// The function is used to pair a CSS class with an associated key (which is mandatory for
/// performance reasons) and wrap it around an HTML element. You can use it for any custom CSS
/// wrapping. If you want to do anything meaningful with this function you'd want to use some
/// associated class or id to track the style.
#[allow(dead_code)]
pub fn html_wrapper(item: Html, key: String, class: Option<String>, id: Option<String>) -> Html {
    html! {
        <div key={key} class={class} id={id}>
            {item}
        </div>
    }
}

pub fn set_iframe_gist(link: &str, height: Option<u32>) -> Html {
    let auto_for_none = || match height {
        Some(_) => None,
        None => Some(String::from("auto")),
    };
    html! {
        <iframe frameborder=0 class={data::IFRAME_GIST}
                scrolling={"no"} seamless={"seamless"}
                style={format!("height: {}px", auto_for_none().unwrap_or(height.unwrap_or(0).add(48).to_string()))}
                srcdoc={format!(
                "<html>
                    <body>
                        <style type=\"text/css\">
                        .gist,
                        .gist-data {{
                            height: {}px;
                        }}
                        </style>
                        <script 
                            src=\"{}\">
                        </script>
                    </body>
                </html>", auto_for_none().unwrap_or(height.unwrap_or(0).to_string()), link.to_owned() + ".js")}
        ></iframe>
    }
}

/// The link has to be an embed link. Example:
/// https://www.youtube-nocookie.com/embed/fwxjMKBMR7s
pub fn set_youtube_iframe(link: &'static str) -> Html {
    html! {
        <div class={"youtube-video-container"}>
                <iframe
                    height="100%"
                    width="100%"
                    src={link}
                    title="YouTube video player"
                    frameborder="0"
                    allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                    referrerpolicy="strict-origin-when-cross-origin"
                    allowfullscreen=true>
                </iframe>
        </div>
    }
}
