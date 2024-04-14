pub mod data;
pub mod error;
pub mod utils;

#[cfg(test)]
pub mod tests {
    use crate::data::{compressed_dir, uncompressed_dir};
    use crate::error::Error;

    #[test]
    pub fn same_dir_sizes() -> Result<(), anyhow::Error> {
        if uncompressed_dir().count() != compressed_dir().count() {
            return Err(Error::UnevenFileCount {
                left_dir: compressed_dir(),
                right_dir: uncompressed_dir(),
                left_dir_count: compressed_dir().count(),
                right_dir_count: uncompressed_dir().count(),
            }
            .into());
        }
        Ok(())
    }
}
