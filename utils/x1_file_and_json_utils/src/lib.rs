pub mod data;
pub mod error;
pub mod utils;

#[cfg(test)]
pub mod tests {
    use std::fs::File;
    use std::io::Read;

    use crate::data::{compressed_dir, uncompressed_dir, JsonArtistCredits, JSON_ARTIST_CREDITS};
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
    #[test]
    fn proper_image_credit_items_count() -> Result<(), anyhow::Error> {
        let mut image_credit_file = File::open(JSON_ARTIST_CREDITS.as_str())?;
        let string_content = {
            let mut temp = String::from("");
            let _ = image_credit_file.read_to_string(&mut temp);
            temp
        };
        // let mut existing_items = vec![];
        let image_credit_json: JsonArtistCredits = serde_json::from_str(&string_content)?;
        for i in 1..=uncompressed_dir().count() {
            if !(image_credit_json.artist_credits.contains_key(&(i as u32))) {
                eprintln!("{:#?}", image_credit_json);
                return Err(Error::MissingArtistCredit { idx: i }.into());
            }
            // let item = image_credit_json.artist_credits.get(&(i as u32)).unwrap();
            // if !item.is_empty() {
            //     existing_items.push(i.to_string());
            // }
        }
        // existing_items.sort();
        // Fail on purpose so I can get the existing_items
        // assert_eq!(vec![String::from("")].join(", "), existing_items.join(", "));
        Ok(())
    }
}
