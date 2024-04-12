#[derive(Hash, Default, PartialEq, Eq, Clone, Debug)]
pub struct ImageElement {
    pub img_src: String,
    pub href: String,
}
#[allow(unused)]
impl ImageElement {
    pub fn new(img_src: Option<String>, href: Option<String>) -> Self {
        Self {
            img_src: img_src.unwrap_or(String::from("")),
            href: href.unwrap_or(String::from("")),
        }
    }
    pub fn from(img_src: String, href: String) -> Self {
        Self { img_src, href }
    }

    pub fn set_img_src(&mut self, img_src: String) {
        self.img_src = img_src;
    }
    pub fn set_href(&mut self, href: String) {
        self.href = href;
    }

    pub fn img_src(self) -> String {
        self.img_src
    }
    pub fn href(self) -> String {
        self.href
    }
}
