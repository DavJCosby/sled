use sled::{SledError, Sled};

fn main() -> Result<(), SledError> {
    let _sled = Sled::new("./cfg/config1.toml")?;

    Ok(())
}
