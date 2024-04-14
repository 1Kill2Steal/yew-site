pub const PAGE_SIZE: i32 = 20;
// You need to have some image pattern alongside the image extension.
pub static IMAGE_NAME_PATTERN: &str = "Hu_Tao_";
pub static COMPRESSED_IMAGE_EXTENSION: &str = ".jpg";
pub static UNCOMPRESSED_IMAGE_EXTENSION: &str = ".png";

// You need to change this if you want to have your images in a different directory
// NOTE: If you do change it you also need to change it in /index.html where all the data trunks
// are copied over.
pub static PICS_COMPRESSED_FOLDER_NAME: &str = "hutao/pics/";
pub static PICS_UNCOMPRESSED_FOLDER_NAME: &str = "hutao/pics_uncompressed/";

pub static JSON_FOLDER_SIZES: &str = "/hutao/json/folder_sizes.json";
#[derive(Clone, PartialEq, serde::Deserialize, Default)]
pub struct JsonFolderSizesLayout {
    // These two are forced to be the same in number from the prerequisite build.sh script btw.
    // (The cargo make utility doesn't compile otherwise)
    compressed_dir_item_count: i32,
    uncompressed_dir_item_count: i32,
}
#[allow(dead_code)]
impl JsonFolderSizesLayout {
    pub fn compressed_count(&self) -> i32 {
        self.compressed_dir_item_count
    }
    pub fn uncompressed_count(&self) -> i32 {
        self.compressed_dir_item_count
    }
}

// CSS PROPERTY NAMES.

pub static FULLSCREEN_OVERLAY_CLASS_NAME: &str = "fullscreen-overlay";
pub static FULLSCREEN_IMG_CLASS_NAME: &str = "fullscreen-img";
