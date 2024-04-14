use crate::data::{
    compressed_dir, target_json_dir, uncompressed_dir, JSON_FOLDER_SIZES, JSON_IMAGE_FILES_NAMES,
    PICS_COMPRESSED_PATHBUF, PICS_UNCOMPRESSED_PATHBUF,
};
use serde_json::json;
use std::fs::File;
use std::io::Write;

fn capture_idx(regex: &regex::Regex, path_item_name: &String) -> u32 {
    let idx_capture = || regex.captures(path_item_name.as_str());
    idx_capture()
        .unwrap()
        .get(2)
        .unwrap_or_else(|| idx_capture().unwrap().get(4).unwrap())
        .as_str()
        .parse::<u32>()
        .unwrap()
}

pub fn set_pics_counts_json() {
    let input = json!({
        "compressed_dir_item_count": compressed_dir().count(),
        "uncompressed_dir_item_count": uncompressed_dir().count(),
    });
    let mut output_json =
        File::create(JSON_FOLDER_SIZES.as_str()).expect("Can't create the JSON file.");
    output_json
        .write_all(input.to_string().as_bytes())
        .expect("Failed to write to the JSON file.");

    println!("{:?}", target_json_dir());
}

pub fn set_pic_names() {
    use std::collections::HashMap;

    let (compressed_item_names, uncompressed_item_names) = {
        // I'm bad at regex :<
        // The regex is for file_name_NUM.extension OR (|) NUM_file_name.extension - You aren't
        // restricted of using other numbers at other spots of the file name. Just don't do stuff
        // like NUM_file_name_NUM.extension because this has 2 NUM captures (gets the last one by
        // default). In the case of a file not having any NUM captures the utility panics which
        // should be your warning to set normal file names bruh. I'm not some wizard that knows
        // what file number you want.
        // https://regex101.com/r/V6HfFq/1
        let file_number_regex = regex::Regex::new(r"(_)(\d+)(.)|(\d+)(_)").unwrap();

        let mut compressed_items_map: HashMap<u32, String> = HashMap::new();
        let mut uncompressed_items_map: HashMap<u32, String> = HashMap::new();
        let get_path_name = |path: std::io::Result<std::fs::DirEntry>,
                             target_pathbuf: &std::path::PathBuf| {
            path.unwrap()
                .path()
                .display()
                .to_string()
                .replace(&target_pathbuf.display().to_string(), "")
        };
        for path in compressed_dir() {
            let path_item_name: String = get_path_name(path, &PICS_COMPRESSED_PATHBUF);
            let captured_idx = || capture_idx(&file_number_regex, &path_item_name);
            compressed_items_map.insert(captured_idx(), path_item_name.clone());
        }
        for path in uncompressed_dir() {
            let path_item_name: String = get_path_name(path, &PICS_UNCOMPRESSED_PATHBUF);
            let captured_idx = || capture_idx(&file_number_regex, &path_item_name);
            uncompressed_items_map.insert(captured_idx(), path_item_name.clone());
        }
        (compressed_items_map, uncompressed_items_map)
    };
    for item in compressed_item_names.iter() {
        println!("ITEM: {:?}", item);
    }
    for item in uncompressed_item_names.iter() {
        println!("ITEM: {:?}", item);
    }
    for idx in compressed_item_names.iter() {
        // Making sure all the valued names are the same so when the user goes to download an image
        // they can download the right uncompressed counterpart.
        assert_eq!(
            &uncompressed_item_names
                .get(idx.0)
                .unwrap()
                .replace(".png", ".jpg"),
            idx.1
        );
    }

    let input = json!({
        "compressed_dir_img_names": compressed_item_names,
        "uncompressed_dir_img_names": uncompressed_item_names,
    });

    let mut output_json =
        File::create(JSON_IMAGE_FILES_NAMES.as_str()).expect("Can't create the JSON file.");
    output_json
        .write_all(input.to_string().as_bytes())
        .expect("Failed to write to the JSON file.");
}
