use crate::utils::*;

pub fn initialize_all() -> Result<(), anyhow::Error> {
    set_pics_counts_json();
    set_pic_names();
    Ok(())
}
