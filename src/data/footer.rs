use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::enums::Socials;
use crate::structs::ImageElement;

static DIRECTORY: &str = "public/socials/";
static EXTENSION: &str = ".svg";

lazy_static! {
    pub static ref SOCIALS_IMAGES: HashMap<Socials, ImageElement> = {
        use Socials::*;

        HashMap::from([
            (
                Twitter,
                ImageElement::from(
                    format!("{DIRECTORY}{}{EXTENSION}", "twitter"),
                    String::from("https://twitter.com/1Kill2Steal"),
                ),
            ),
            (
                GitHub,
                ImageElement::from(
                    format!("{DIRECTORY}{}{EXTENSION}", "github"),
                    String::from("https://github.com/1Kill2Steal"),
                ),
            ),
            (
                Discord,
                ImageElement::from(
                    format!("{DIRECTORY}{}{EXTENSION}", "discord"),
                    String::from("https://discord.gg/d8eyqpK2PN"),
                ),
            ),
        ])
    };
}
