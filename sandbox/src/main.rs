use sled::{SLEDError, SLED};

fn main() -> Result<(), SLEDError> {
    let _sled = SLED::new("./cfg/config1.toml")?;

    Ok(())
}
