use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "atusb")]
#[command(about = "All-in-one USB management tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all connected USB devices
    List,
    /// Check USB health (SMART status)
    Health { device: String },
    /// Flash an ISO to a USB drive
    Flash { iso: String, usb: String },
}
