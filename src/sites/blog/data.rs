pub const BLOG_SHOWCASE: &'static str = "blog-showcase";

pub const ID_REF: &'static str = "#";
pub const BLOG_METADATA: &'static str = "blog-metadata";
pub const BLOG_LINK: &'static str = "https://1kill2steal.netlify.app/blog/";
pub const TEX_ITEM_WIDTH: &'static str = "var(--tex-item-width)";
pub const DETAILS_ITEM_GLYPH: &'static str = "nerd-font-glyph nf-md-subdirectory_arrow_right";
pub const BLOG_DETAILS_ITEM: &'static str = "blog-details-item";

pub struct SectionItem {
    name: String,
    link: String,
}
impl SectionItem {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name).replace('_', ""),
            link: String::from(BLOG_LINK) + name,
        }
    }
}
