mod cli;
mod usb;
mod health;
mod flash;

use anyhow::Result;
use cli::{Cli, Commands};
use clap::Parser;  // Add this import

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();  // This will now work

    match cli.command {
        Commands::List => {
            let usb_devices = usb::list_usb_devices()?;
            println!("Detected USB Devices:");
            for device in usb_devices {
                println!("- {} ({}): {} bytes", device.name, device.path, device.size);
            }
        }
        Commands::Health { device } => {
            let health = health::check_usb_health(&device)?;
            println!("{}", health);
        }
        Commands::Flash { iso, usb } => {
            flash::flash_iso(&iso, &usb)?;
            println!("ISO flashed successfully to {}", usb);
        }
    }

    Ok(())
}
