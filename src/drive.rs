use clap::{ValueEnum};
use std::process::{Command};
use anyhow::{Result, Context};
use std::path::Path;

use crate::error::{UpvError, EXIT_UPV_ERROR};

#[derive(Debug, Clone, ValueEnum)]
pub enum UPVDomain {
    ALUMNO,
    UPVNET,
}

impl std::fmt::Display for UPVDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UPVDomain::ALUMNO => write!(f, "ALUMNO"),
            UPVDomain::UPVNET => write!(f, "UPVNET"),
        }
    }
}

pub struct DriveManager;

impl DriveManager {
    /// Mounts the UPV Personal Network Drive (Disco W) to a specified drive letter
    pub fn mount(username: &str, domain: &UPVDomain, password: Option<&str>, drive: char, open_explorer: bool) -> Result<()> {
        println!("Mounting Disco W to drive {}:...", drive);
        
        let first_letter = username.chars().next()
            .context("Username cannot be empty")?
            .to_lowercase()
            .to_string();
        
        let server_path = match domain {
            UPVDomain::ALUMNO => format!(r"\\nasupv.upv.es\alumnos\{}\{}", first_letter, username),
            UPVDomain::UPVNET => format!(r"\\nasupv.upv.es\discos\{}\{}", first_letter, username),
        };
        
        let mut cmd = Command::new("net");
        cmd.arg("use")
           .arg(format!("{}:", drive))
           .arg(&server_path);
        
        // Only add /USER if password is provided
        if let Some(pwd) = password {
            cmd.arg(format!("/user:{}\\{}", domain, username))
               .arg(pwd);
        }
        
        let output = cmd.output()
            .context("Failed to execute net use command")?;
        
        if output.status.success() {
            println!("Disco W mounted successfully to drive {}:", drive);
            
            // Open in Explorer if requested
            if open_explorer {
                Self::open_drive(drive, false)?;
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(UpvError::new(
                format!("Failed to mount drive {}: {}", drive, error),
                EXIT_UPV_ERROR
            ).into());
        }
        
        Ok(())
    }
    
    /// Opens the specified drive in Windows Explorer
    pub fn open_drive(drive: char, check_if_exists: bool) -> Result<()> {
        let path = format!("{}:\\", drive);

        if check_if_exists && !Path::new(&path).exists() {
            return Err(UpvError::new(
                format!("Drive {} does not exist", drive),
                EXIT_UPV_ERROR
            ).into());
        }

        println!("Opening drive {}: in Explorer...", drive);
        Command::new("explorer.exe")
            .arg(&path)
            .spawn()
            .context("Failed to launch Explorer")?;

        Ok(())
    }
    
    /// Unmounts the network drive
    pub fn unmount(drive: char, force: bool) -> Result<()> {
        println!("Unmounting drive {}:...", drive);
        
        let mut cmd = Command::new("net");
        cmd.arg("use")
           .arg(format!("{}:", drive))
           .arg("/delete");
        
        // Only add /y if force is true
        if force {
            cmd.arg("/y");
        }
        
        let output = cmd.output()
            .context("Failed to execute net use delete command")?;
        
        if output.status.success() {
            println!("Drive {}: unmounted successfully", drive);
        } else {
            let stdout = String::from_utf8_lossy(&output.stdout);
            
            // If stdout contains "/N" it's part of "(Y/N)". This confirmation shows when it's trying to unmount a drive that is in use
            // (files are open, the folder is open, etc.)
            if stdout.contains("/N") {
                return Err(UpvError::new(
                    format!("Drive {}: is currently IN USE. Please CLOSE any open files or folders on this drive and try again, or run this again with the --force option to unmount it anyways, accepting that INFORMATION COULD BE LOST.", drive),
                    EXIT_UPV_ERROR
                ).into());
            }

            let error = String::from_utf8_lossy(&output.stderr);
            return Err(UpvError::new(
                format!("Failed to unmount drive {}: {}", drive, error),
                EXIT_UPV_ERROR
            ).into());
        }
        
        Ok(())
    }
    
    /// Checks the status of the network drive by listing all network drives
    pub fn status() -> Result<()> {
        println!("Checking network drive status...");
        
        let output = Command::new("net")
            .arg("use")
            .output()
            .context("Failed to check drive status")?;
        
        let status = String::from_utf8_lossy(&output.stdout);
        println!("{}", status);
        
        Ok(())
    }
}