use std::process::Command;

pub fn check_usb_health(device_path: &str) -> anyhow::Result<String> {
    let output = Command::new("smartctl")
        .arg("-i")
        .arg(device_path)
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    } else {
        Err(anyhow::anyhow!(
            "SMART check failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
