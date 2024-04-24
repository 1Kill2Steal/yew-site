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

pub fn set_iframe_gist(link: &str, height: Option<&str>) -> Html {
    html! {
        <iframe frameborder=0 class={data::IFRAME_GIST}
                scrolling={"no"} seamless={"seamless"}
                srcdoc={format!(
                "<html>
                    <body>
                        <style type=\"text/css\">
                        .gist,
                        .gist-data {{
                            height: {};
                        }}
                        </style>
                        <script 
                            src=\"{}\">
                        </script>
                    </body>
                </html>", height.unwrap_or("auto"), link.to_owned() + ".js")}
        ></iframe>
    }
}

pub fn set_youtube_iframe(iframe: Html) -> Html {
    html! {
        <div class={"youtube-video-container"}>
            {iframe}
        </div>
    }
}
