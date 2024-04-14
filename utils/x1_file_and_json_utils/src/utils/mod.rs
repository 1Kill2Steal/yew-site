use crate::data::{compressed_dir, target_json_dir, uncompressed_dir, JSON_FULL_PATH_AND_NAME};
use serde_json::json;

pub fn set_pics_json() {
    use std::fs::File;
    use std::io::Write;
    let input = json!({
        "compressed_dir_item_count": compressed_dir().count(),
        "uncompressed_dir_item_count": uncompressed_dir().count(),
    });
    let mut output_json =
        File::create(JSON_FULL_PATH_AND_NAME.as_str()).expect("Can't create the JSON file.");
    output_json
        .write_all(input.to_string().as_bytes())
        .expect("Failed to write to the JSON file.");

    println!("{:?}", target_json_dir());
}
