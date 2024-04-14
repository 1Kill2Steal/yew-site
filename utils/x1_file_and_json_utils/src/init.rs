use crate::utils::set_pics_json;

pub fn initialize_all() -> Result<(), anyhow::Error> {
    set_pics_json();
    Ok(())
}
