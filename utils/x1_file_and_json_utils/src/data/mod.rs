use lazy_static::lazy_static;
use std::fs;
use std::fs::ReadDir;
use std::path::PathBuf;

lazy_static! {
    pub static ref GIT_ROOT_PATH: PathBuf = {
        let mut cargo_toml_dir = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));

        // Go back 2 dirs from the Cargo.toml directory
        cargo_toml_dir.pop();
        cargo_toml_dir.pop();

        cargo_toml_dir
    };
    pub static ref PICS_PATHBUF: PathBuf = {
        let dir_name = "hutao";
        PathBuf::from(format!("{}/{}", GIT_ROOT_PATH.display(), dir_name))
    };
    pub static ref PICS_JSON_PATHBUF: PathBuf = {
        let target = "json";
        PathBuf::from(format!("{}/{}", PICS_PATHBUF.display(), target))
    };
    pub static ref JSON_FULL_PATH_AND_NAME: String = {
        let name = "folder_sizes.json";
        format!("{}/{}", PICS_JSON_PATHBUF.display(), name)
    };
    pub static ref JSON_FILE: fs::File = {
        let file =
        fs::File::open(JSON_FULL_PATH_AND_NAME.as_str());
        if file.is_err() {
            eprintln!("Incorrect file name.");
        }
        file.unwrap()
    };
    pub static ref PICS_UNCOMPRESSED_PATHBUF: PathBuf = {
        let target = "pics_uncompressed";
        PathBuf::from(format!("{}/{}", PICS_PATHBUF.display(), target))
    };
    pub static ref PICS_COMPRESSED_PATHBUF: PathBuf = {
        let target = "pics";
        PathBuf::from(format!("{}/{}", PICS_PATHBUF.display(), target))
    };

}

pub fn target_json_dir() -> ReadDir {
    fs::read_dir(format!("{}", PICS_JSON_PATHBUF.display()))
        .expect("failed to read target_json_dir")
}
pub fn uncompressed_dir() -> ReadDir {
    fs::read_dir(format!("{}", PICS_UNCOMPRESSED_PATHBUF.display()))
        .expect("failed to read uncompressed_dir")
}
pub fn compressed_dir() -> ReadDir {
    fs::read_dir(format!("{}", PICS_COMPRESSED_PATHBUF.display()))
        .expect("failed to read compressed_dir")
}
