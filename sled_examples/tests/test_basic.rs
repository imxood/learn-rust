use std::str::FromStr;

use anyhow::Result;

#[derive(Debug, Seria)]
struct Device {
    ip: String,
    port: u16,
}

impl Device {
    fn new() -> Self {
        Self {
            ip: "127.0.0.1".into(),
            port: 12345,
        }
    }
}

#[test]
fn test_basic() -> Result<()> {
    let db = sled::open("welcome-to-sled")?;
    db.update_and_fetch("device", |value_opt| match value_opt {
        Some(existing) => {}
        None => {}
    });
    Ok(())
}
