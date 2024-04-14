use std::fs::ReadDir;
use thiserror::Error;
// If the Error type is unused in general it's definitely used in the tests at
// least. This is to stop the lint error.
#[allow(unused)]
#[derive(Error, Debug)]
pub enum Error {
    #[error("! Uneven file count in directories {left_dir:?} and {right_dir:?} ! \n(expected {left_dir_count:?} == {right_dir_count:?})")]
    UnevenFileCount {
        left_dir: ReadDir,
        right_dir: ReadDir,
        left_dir_count: usize,
        right_dir_count: usize,
    },
}
