// Initial WIP version - needs to be completed and tested
// This code is a work in progress (WIP) and may not be fully functional yet.

// This CLI tool manages UPV's VPN connection and Personal Network Drive (Disco W) on Windows.
// It allows users to create, connect, disconnect, and check the status of VPN connections,
// as well as mount, unmount, and check the status of the personal network drive.
// It uses PowerShell commands for VPN management and Windows commands for network drive operations.

// Dependencies:
// - clap: For command-line argument parsing
// - anyhow: For error handling

use clap::{Parser, Subcommand, ValueEnum};
use std::process::{Command, Stdio};
use std::io::Write;
use anyhow::{Result, Context};

// Embed the EAP configuration XML file at compile time
const EAP_CONFIG_XML: &str = include_str!("../resources/UPV_Config.xml");

#[derive(Debug, Clone, ValueEnum)]
enum UPVDomain {
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

#[derive(Parser)]
#[command(name = "upv-cli")]
#[command(about = "CLI tool to manage UPV's VPN connection and Personal Network Drive (Disco W)")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
}

#[derive(Subcommand)]
enum VpnAction {
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
    /// Check VPN connection status
    Status,
}

#[derive(Subcommand)]
enum DriveAction {
    /// Mount the personal network drive (Disco W)
    Mount {
        /// UPV domain
        #[arg(value_enum)]
        domain: UPVDomain,
        /// Username for network drive
        #[arg(short, long)]
        username: String,
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
    },
    /// Check network drive status
    Status,
}

struct VpnManager;
struct DriveManager;

impl VpnManager {
    fn create(name: &str, auto_connect: bool) -> Result<()> {
        println!("Creating VPN connection '{}'...", name);
        
        let server_address = "vpn.upv.es";
        
        // Clean the XML content and create here-string like your .NET approach
        let xml_content = EAP_CONFIG_XML.trim().trim_start_matches('\u{feff}'); // Remove BOM if present
        
        let ps_command = format!(
            "Add-VpnConnection -Name '{}' -ServerAddress '{}' -AuthenticationMethod Eap -EncryptionLevel Required -TunnelType Sstp -EapConfigXmlStream @'\r\n{}\r\n'@\r\n\r\n",
            name,
            server_address,
            xml_content
        );
        
        // Execute PowerShell with command via stdin
        let mut child = Command::new("powershell")
            .arg("-Command")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("Failed to spawn PowerShell process")?;
        
        // Write command to stdin and close it
        if let Some(stdin) = child.stdin.take() {
            let mut stdin = stdin;
            stdin.write_all(ps_command.as_bytes())
                .context("Failed to write to PowerShell stdin")?;
            // stdin is automatically closed when it goes out of scope
        }
        
        let output = child.wait_with_output()
            .context("Failed to wait for PowerShell command")?;
        
        if output.status.success() {
            println!("VPN connection '{}' created successfully", name);
            
            // Auto-connect if requested
            if auto_connect {
                Self::connect(name)?;
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Failed to create VPN connection: {}", error);
        }
        
        Ok(())
    }
    
    fn connect(name: &str) -> Result<()> {
        println!("Opening connection dialog for '{}'...", name);
        
        // Use rasphone to open the connection dialog
        let output = Command::new("rasphone")
            .arg("-d")
            .arg(name)
            .output()
            .context("Failed to execute rasphone command")?;
        
        if output.status.success() {
            println!("Connection dialog opened for '{}'", name);
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Failed to open connection dialog: {}", error);
        }
        
        Ok(())
    }
    
    fn disconnect() -> Result<()> {
        println!("Disconnecting from VPN...");
        
        let output = Command::new("rasdial")
            .arg("/disconnect")
            .output()
            .context("Failed to execute rasdial disconnect")?;
        
        if output.status.success() {
            println!("Disconnected from VPN successfully");
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Failed to disconnect: {}", error);
        }
        
        Ok(())
    }
    
    fn status() -> Result<()> {
        println!("Checking VPN status...");
        
        let output = Command::new("rasdial")
            .output()
            .context("Failed to check VPN status")?;
        
        let status = String::from_utf8_lossy(&output.stdout);
        println!("{}", status);
        
        Ok(())
    }
}

impl DriveManager {
    fn mount(domain: &UPVDomain, username: &str, drive: char, open_explorer: bool) -> Result<()> {
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
           .arg(&server_path)
           .arg(format!("/user:{}", username));
        
        let output = cmd.output()
            .context("Failed to execute net use command")?;
        
        if output.status.success() {
            println!("Disco W mounted successfully to drive {}:", drive);
            
            // Open in Explorer if requested
            if open_explorer {
                Self::open_drive(drive)?;
            }
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Failed to mount drive: {}", error);
        }
        
        Ok(())
    }
    
    fn open_drive(drive: char) -> Result<()> {
        println!("Opening drive {}: in Explorer...", drive);
        
        let output = Command::new("explorer.exe")
            .arg(format!("{}:", drive))
            .output()
            .context("Failed to open Explorer")?;
        
        if output.status.success() {
            println!("Drive {}: opened in Explorer", drive);
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Failed to open Explorer: {}", error);
        }
        
        Ok(())
    }
    
    fn unmount(drive: char) -> Result<()> {
        println!("Unmounting drive {}:...", drive);
        
        let output = Command::new("net")
            .arg("use")
            .arg(format!("{}:", drive))
            .arg("/delete")
            .output()
            .context("Failed to execute net use delete command")?;
        
        if output.status.success() {
            println!("Drive {}: unmounted successfully", drive);
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            eprintln!("Failed to unmount drive: {}", error);
        }
        
        Ok(())
    }
    
    fn status() -> Result<()> {
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
                VpnAction::Status => {
                    VpnManager::status()?;
                }
            }
        }
        Commands::Drive { action } => {
            match action {
                DriveAction::Mount { domain, username, drive, open } => {
                    DriveManager::mount(&domain, &username, drive, open)?;
                }
                DriveAction::Unmount { drive } => {
                    DriveManager::unmount(drive)?;
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
// upv-cli vpn create "My UPV Connection" --connect
// upv-cli vpn create "UPV Work" -c  # Short flag for --connect
// upv-cli vpn connect "My UPV Connection"
// upv-cli vpn disconnect
// upv-cli vpn status
// upv-cli drive mount UPVNET --username myuser --drive W --open
// upv-cli drive mount ALUMNO -u myuser -d W -o  # Short flags
// upv-cli drive unmount --drive W
// upv-cli drive status