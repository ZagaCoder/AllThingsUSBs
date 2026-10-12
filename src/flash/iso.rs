use std::process::Command;

pub fn flash_iso(iso_path: &str, usb_path: &str) -> anyhow::Result<()> {
    let status = Command::new("dd")
        .arg("if=".to_owned() + iso_path)
        .arg("of=".to_owned() + usb_path)
        .arg("bs=4M")
        .arg("status=progress")
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to flash ISO"))
    }
}
