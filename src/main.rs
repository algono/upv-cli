// upv-cli: A CLI tool for managing VPN and network shares from UPV (Universitat Politècnica de València) on Windows.

// This CLI tool manages UPV's VPN connection and Personal Network Drive (Disco W) on Windows.
// It allows users to create, connect, disconnect, and check the status of VPN connections,
// as well as mount, unmount, and check the status of the personal network drive.
// It uses PowerShell commands for VPN management and Windows commands for network drive operations.

// Dependencies:
// - clap: For command-line argument parsing
// - anyhow: For error handling

mod cli;
mod drive;
mod vpn;

use clap::Parser;
use anyhow::Result;
use cli::{Cli, Commands, VpnAction, DriveAction};
use drive::DriveManager;
use vpn::VpnManager;

fn main() -> Result<()> {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Vpn { action } => {
            match action {
                VpnAction::Create { name, connect } => {
                    VpnManager::create(&name, connect)?;
                }
                VpnAction::Connect { name } => {
                    VpnManager::connect(&name)?;
                }
                VpnAction::Disconnect => {
                    VpnManager::disconnect()?;
                }
                VpnAction::Delete { name, force } => {
                    VpnManager::delete(&name, force)?;
                }
                VpnAction::List => {
                    VpnManager::list()?;
                }
                VpnAction::Purge { force, except } => {
                    VpnManager::purge(force, except)?;
                }
                VpnAction::Status => {
                    VpnManager::status()?;
                }
            }
        }
        Commands::Drive { action } => {
            match action {
                DriveAction::Mount { username, domain, password, drive, open } => {
                    DriveManager::mount(&username, &domain, password.as_deref(), drive, open)?;
                }
                DriveAction::Unmount { drive, force } => {
                    DriveManager::unmount(drive, force)?;
                }
                DriveAction::Open { drive } => {
                    DriveManager::open_drive(drive, true)?;
                }
                DriveAction::Status => {
                    DriveManager::status()?;
                }
            }
        }
    }
    
    Ok(())
}

// Usage examples:
// upv vpn create "My UPV Connection" --connect
// upv vpn create "UPV Work" -c  # Short flag for --connect
// upv vpn connect "My UPV Connection"
// upv vpn disconnect
// upv vpn delete "My UPV Connection"
// upv vpn delete "UPV Work" --force  # Skip confirmation
// upv vpn list
// upv vpn purge                       # Delete all UPV connections (with double confirmation)
// upv vpn purge --force              # Delete all UPV connections without confirmation
// upv vpn purge --except "Keep This" # Delete all except specified connections
// upv vpn purge -e "VPN1" -e "VPN2"  # Delete all except VPN1 and VPN2
// upv vpn status
// upv drive mount myuser UPVNET --drive W --open  # Uses VPN credentials
// upv drive mount myuser UPVNET --password mypass --drive W --open  # Uses explicit credentials
// upv drive mount myuser ALUMNO -d W -o  # Short flags, uses VPN credentials
// upv drive mount myuser ALUMNO -p mypass -d W -o  # Short flags with password
// upv drive unmount --drive W
// upv drive status