use yew_router::hooks::use_navigator;

use super::*;

pub mod blog_nav;
pub mod contents;
pub mod utils;

/// Re-exports for convenience sake.
use {contents::*, utils::*};

#[function_component(BlogUnderstandingBigONotation)]
pub fn understanding_big_o_notation() -> Html {
    let nav = use_navigator().unwrap();
    let clipboard = yew_hooks::use_clipboard();
    wrap_blog_subsite(nav, blog_understaning_big_o_notation_contents(clipboard))
}
