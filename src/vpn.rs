use std::process::{Command, Stdio};
use std::io::{self, Write};
use anyhow::{Result, Context};

use crate::error::{UpvError, EXIT_UPV_ERROR};

// Embed the EAP configuration XML file at compile time
const EAP_CONFIG_XML: &str = include_str!("../resources/UPV_Config.xml");

pub struct VpnManager;

impl VpnManager {
    // Private utility functions

    /// Retrieves all UPV VPN connections by filtering based on the server address.
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
            return Err(UpvError::new(
                format!("Failed to get VPN connections: {}", error),
                EXIT_UPV_ERROR
            ).into());
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let connections: Vec<String> = stdout.lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        
        Ok(connections)
    }
    
    /// Deletes a VPN connection by name using PowerShell.
    fn delete_connection(name: &str) -> Result<()> {
        let ps_command = format!("Remove-VpnConnection -Name '{}' -Force", name);
        
        let output = Command::new("powershell")
            .arg("-Command")
            .arg(&ps_command)
            .output()
            .context("Failed to execute PowerShell command")?;
        
        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(UpvError::new(
                format!("Failed to delete VPN connection '{}': {}", name, error),
                EXIT_UPV_ERROR
            ).into());
        }
        
        Ok(())
    }

    // Public methods for VPN management

    /// Creates a new UPV VPN connection with the specified name and optional auto-connect.
    pub fn create(name: &str, auto_connect: bool) -> Result<()> {
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
            return Err(UpvError::new(
                format!("Failed to create VPN connection '{}': {}", name, error),
                EXIT_UPV_ERROR
            ).into());
        }
        
        Ok(())
    }
    
    /// Purges all UPV VPN connections, with optional exceptions and force confirmation.
    pub fn purge(force: bool, except_names: Vec<String>) -> Result<()> {
        // Get the list of UPV connections
        let all_connections = Self::get_upv_connections()
            .context("Failed to retrieve UPV VPN connections")?;
        
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
    
    /// Connects to an existing UPV VPN connection using rasphone.
    pub fn connect(name: &str) -> Result<()> {
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
            return Err(UpvError::new(
                format!("Failed to open connection dialog for '{}': {}", name, error),
                EXIT_UPV_ERROR
            ).into());
        }
        
        Ok(())
    }
    
    /// Disconnects from the current UPV VPN connection using rasdial.
    pub fn disconnect() -> Result<()> {
        println!("Disconnecting from VPN...");
        
        let output = Command::new("rasdial")
            .arg("/disconnect")
            .output()
            .context("Failed to execute rasdial disconnect")?;
        
        if output.status.success() {
            println!("Disconnected from VPN successfully");
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(UpvError::new(
                format!("Failed to disconnect from VPN: {}", error),
                EXIT_UPV_ERROR
            ).into());
        }
        
        Ok(())
    }
    
    /// Deletes a specific UPV VPN connection by name, with optional confirmation.
    pub fn delete(name: &str, force: bool) -> Result<()> {
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
        
        Self::delete_connection(name)?;

        println!("VPN connection '{}' deleted successfully", name);
        
        Ok(())
    }
    
    /// Lists all UPV VPN connections.
    pub fn list() -> Result<()> {
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
    
    /// Checks the status of the current VPN connection using rasdial.
    pub fn status() -> Result<()> {
        println!("Checking VPN status...");
        
        let output = Command::new("rasdial")
            .output()
            .context("Failed to check VPN status")?;
        
        let status = String::from_utf8_lossy(&output.stdout);
        println!("{}", status);
        
        Ok(())
    }
}