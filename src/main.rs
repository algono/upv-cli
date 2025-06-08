// upv-cli: A CLI tool for managing VPN and network shares from UPV (Universitat Politècnica de València) on Windows.

// This CLI tool manages UPV's VPN connection and Personal Network Drive (Disco W) on Windows.
// It allows users to create, connect, disconnect, and check the status of VPN connections,
// as well as mount, unmount, and check the status of the personal network drive.
// It uses PowerShell commands for VPN management and Windows commands for network drive operations.

// Dependencies:
// - clap: For command-line argument parsing
// - anyhow: For error handling

use clap::{Parser, Subcommand, ValueEnum};
use std::process::{Command, Stdio};
use std::io::{self, Write};
use anyhow::{Result, Context};
use std::path::Path;

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
#[command(name = "upv")]
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
enum DriveAction {
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

struct VpnManager;
struct DriveManager;

impl VpnManager {
    fn get_upv_connections() -> Result<Vec<String>> {
        let server_address = "vpn.upv.es";
        let ps_command = format!(
            "Get-VpnConnection | Where-Object {{$_.ServerAddress -eq '{}'}} | Select-Object -ExpandProperty Name",
            server_address
        );
        
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(&ps_command)
            .output()
            .context("Failed to execute PowerShell command")?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to get VPN connections: {}", error));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let connections: Vec<String> = stdout.lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        
        Ok(connections)
    }
    
    fn delete_connection(name: &str) -> Result<()> {
        let ps_command = format!("Remove-VpnConnection -Name '{}' -Force", name);
        
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(&ps_command)
            .output()
            .context("Failed to execute PowerShell command")?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("Failed to delete VPN connection '{}': {}", name, error));
        }
        
        Ok(())
    }
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
    
    fn purge(force: bool, except_names: Vec<String>) -> Result<()> {
        // Get the list of UPV connections
        let all_connections = match Self::get_upv_connections() {
            Ok(conns) => conns,
            Err(e) => {
                eprintln!("Failed to get VPN connections: {}", e);
                return Ok(());
            }
        };
        
        if all_connections.is_empty() {
            println!("No UPV VPN connections found to delete.");
            return Ok(());
        }
        
        // Filter out the connections to except (only if there are exceptions)
        let connections = if except_names.is_empty() {
            all_connections
        } else {
            all_connections
                .into_iter()
                .filter(|conn| !except_names.contains(conn))
                .collect()
        };
        
        if connections.is_empty() {
            println!("No UPV VPN connections found to delete.");
            return Ok(());
        }
        
        // Show what will be deleted
        println!("Found {} UPV VPN connection(s) to delete:", connections.len());
        for conn in &connections {
            println!("  - {}", conn);
        }
        
        if !force {
            // First confirmation
            print!("\nAre you sure you want to delete ALL {} UPV VPN connections? (y/N): ", connections.len());
            io::stdout().flush().context("Failed to flush stdout")?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).context("Failed to read user input")?;
            
            let confirmation = input.trim().to_lowercase();
            if confirmation != "y" && confirmation != "yes" {
                println!("Operation cancelled.");
                return Ok(());
            }
            
            // Second confirmation (extra safety)
            print!("This action cannot be undone. Type 'DELETE' to confirm: ");
            io::stdout().flush().context("Failed to flush stdout")?;
            
            let mut input2 = String::new();
            io::stdin().read_line(&mut input2).context("Failed to read user input")?;
            
            if input2.trim() != "DELETE" {
                println!("Operation cancelled.");
                return Ok(());
            }
        }
        
        println!("\nDeleting {} UPV VPN connections...", connections.len());
        
        let mut deleted_count = 0;
        let mut failed_count = 0;
        
        for connection in connections {
            match Self::delete_connection(&connection) {
                Ok(()) => {
                    println!("  ✓ Deleted '{}'", connection);
                    deleted_count += 1;
                }
                Err(e) => {
                    eprintln!("  ✗ Failed to delete '{}': {}", connection, e);
                    failed_count += 1;
                }
            }
        }
        
        println!("\nPurge completed:");
        println!("  {} connections deleted successfully", deleted_count);
        if failed_count > 0 {
            println!("  {} connections failed to delete", failed_count);
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
    
    fn delete(name: &str, force: bool) -> Result<()> {
        if !force {
            print!("Are you sure you want to delete VPN connection '{}'? (y/N): ", name);
            io::stdout().flush().context("Failed to flush stdout")?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).context("Failed to read user input")?;
            
            let confirmation = input.trim().to_lowercase();
            if confirmation != "y" && confirmation != "yes" {
                println!("Operation cancelled.");
                return Ok(());
            }
        }
        
        println!("Deleting VPN connection '{}'...", name);
        
        match Self::delete_connection(name) {
            Ok(()) => println!("VPN connection '{}' deleted successfully", name),
            Err(e) => eprintln!("Failed to delete VPN connection: {}", e),
        }
        
        Ok(())
    }
    
    fn list() -> Result<()> {
        println!("Listing UPV VPN connections...");
        
        let connections = Self::get_upv_connections()?;
        
        if connections.is_empty() {
            println!("No UPV VPN connections found.");
        } else {
            println!("Found {} UPV VPN connection(s):", connections.len());
            for conn in connections {
                println!("  - {}", conn);
            }
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
    fn mount(username: &str, domain: &UPVDomain, password: Option<&str>, drive: char, open_explorer: bool) -> Result<()> {
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
            eprintln!("Failed to mount drive: {}", error);
        }
        
        Ok(())
    }
    
    fn open_drive(drive: char, check_if_exists: bool) -> Result<()> {
        let path = format!("{}:\\", drive);

        if check_if_exists && !Path::new(&path).exists() {
            anyhow::bail!("Drive {} does not exist", drive);
        }

        println!("Opening drive {}: in Explorer...", drive);
        Command::new("explorer.exe")
            .arg(&path)
            .spawn()
            .context("Failed to launch Explorer")?;

        Ok(())
    }
    
    fn unmount(drive: char, force: bool) -> Result<()> {
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
                eprintln!("Drive {}: is currently IN USE. Please CLOSE any open files or folders on this drive and try again, or run this again with the --force option to unmount it anyways, accepting that INFORMATION COULD BE LOST.", drive);
                return Ok(());
            }

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
// upv drive mount UPVNET --username myuser --drive W --open  # Uses VPN credentials
// upv drive mount UPVNET --username myuser --password mypass --drive W --open  # Uses explicit credentials
// upv drive mount ALUMNO -u myuser -d W -o  # Short flags, uses VPN credentials
// upv drive mount ALUMNO -u myuser -p mypass -d W -o  # Short flags with password
// upv drive unmount --drive W
// upv drive status