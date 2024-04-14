mod data;
mod error;
mod init;
mod utils;

fn main() -> Result<(), anyhow::Error> {
    init::initialize_all()
}
