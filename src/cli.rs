use crate::drive::UPVDomain;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "upv")]
#[command(about = "CLI tool to manage UPV's VPN connection and Personal Network Drive (Disco W)")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// VPN connection management
    Vpn {
        #[command(subcommand)]
        action: VpnAction,
    },
    /// Personal Network Drive (Disco W) management
    Drive {
        #[command(subcommand)]
        action: DriveAction,
    },
    /// Generate a PowerShell auto-completions script that you can source in your PowerShell profile
    // (this could allow other shells, but for now since we only support Windows, we can keep it simple)
    Completions
}

#[derive(Subcommand)]
pub enum VpnAction {
    /// Create a new UPV VPN connection
    Create {
        /// Name for the VPN connection
        name: String,
        /// Connect immediately after creating
        #[arg(short, long)]
        connect: bool,
    },
    /// Connect to an existing UPV VPN using rasphone
    Connect {
        /// Name of the VPN connection to connect to
        name: String,
    },
    /// Disconnect from UPV VPN
    Disconnect,
    /// Delete an existing UPV VPN connection
    Delete {
        /// Name of the VPN connection to delete
        name: String,
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
    /// List all UPV VPN connections
    List,
    /// Delete ALL UPV VPN connections (with double confirmation)
    Purge {
        /// Skip confirmation prompts
        #[arg(short, long)]
        force: bool,
        /// VPN connection names to exclude from deletion (can be used multiple times)
        #[arg(short, long = "except", value_name = "NAME")]
        except: Vec<String>,
    },
    /// Check VPN connection status
    Status,
}

#[derive(Subcommand)]
pub enum DriveAction {
    /// Mount the personal network drive (Disco W)
    Mount {
        /// Your UPV username (example: if your email is "user@upv.es", your username is "user")
        username: String,

        /// UPV domain
        #[arg(value_enum, ignore_case = true)]
        domain: UPVDomain,
        /// Password for network drive (if not provided, uses current VPN or Wi-Fi credentials)
        #[arg(short, long)]
        password: Option<String>,
        /// Drive letter to mount to
        #[arg(short, long, default_value = "W")]
        drive: char,
        /// Open the drive in Explorer after mounting
        #[arg(short, long)]
        open: bool,
    },
    /// Unmount the personal network drive
    Unmount {
        /// Drive letter to unmount
        #[arg(short, long, default_value = "W")]
        drive: char,
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
    /// Open the personal network drive in Explorer
    Open {
        /// Drive letter to open
        #[arg(short, long, default_value = "W")]
        drive: char,
    },
    /// Check network drive status
    Status,
}