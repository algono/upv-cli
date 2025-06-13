# upv-cli

**A command-line tool for managing VPN and network shares (on Windows) from UPV (Universitat Politècnica de València), built in Rust.**

[![GitHub](https://img.shields.io/badge/github-algono%2Fupv--cli-8da0cb?logo=github)](https://github.com/algono/upv-cli)
[![Crates.io](https://img.shields.io/crates/v/upv-cli.svg)](https://crates.io/crates/upv-cli)
[![Rust](https://img.shields.io/badge/Rust-D34516?logo=rust&logoColor=white)](https://www.rust-lang.org/)


---

## 🚀 Features

- Setup UPV's VPN configuration on Windows easily via CLI (`upv vpn create <NAME>`)
- Mount and unmount your personal UPV network drive (colloquially known as "_Disco W_")
- Automatically open the drive after mounting
- Force unmount even if files are in use
- Built-in `open` command to directly open mounted drives
- Fast and lightweight — no GUI required
- Friendly for PowerShell, CMD, and other shells

---

## 📦 Installation

> ⚠️ **Important:** This tool is only available for **Windows**, since it uses Windows-specific features like `net use` and `rasdial`.

> 📝 **Note:** After installing, the binary should be available as `upv` (the file is `upv.exe`).

### From Scoop

If you use [Scoop](https://scoop.sh/), you can install `upv-cli` by adding my personal bucket with the following commands:

```bash
scoop bucket add algono https://github.com/algono/scoop-algono
scoop install upv-cli
```

### From GitHub Releases

You can download the latest release's binary file from the [GitHub Releases page](https://github.com/algono/upv-cli/releases/latest).

Click here to directly download the latest release: [upv-cli latest release](https://github.com/algono/upv-cli/releases/latest/download/upv.exe).

Just download the `upv.exe` file, place it in a directory that's in your `PATH`, and you're good to go!

### From Crates\.io

> **Note:** Requires Rust and Cargo installed. If you don't have them, follow the [Rust installation guide](https://www.rust-lang.org/tools/install).

If you have cargo installed, you can install it from [crates.io](https://crates.io/crates/upv-cli):

```bash
cargo install upv-cli
```

Or from source after cloning the repository:

```bash
git clone https://github.com/algono/upv-cli
cd upv-cli
cargo install --path .
```

Make sure `~/.cargo/bin` is in your `PATH`.

---

## ⚡ Quickstart

```bash
upv vpn create UPV --connect
upv drive mount myuser UPVNET -d W -o
```

### Explanation

```bash
# Create and connect to a VPN named "UPV"
upv vpn create UPV --connect
# Mount the UPV network drive (Disco W) using VPN credentials for the user "myuser" in the "UPVNET" domain to drive W, and then open it from File Explorer
upv drive mount myuser UPVNET -d W -o
```

---

## 🧑‍💻 Usage

```bash
upv <command> [options]
```

### Example commands:

```bash
upv vpn create "My UPV Connection" --connect
upv vpn create "UPV Work" -c  # Short flag for --connect
upv vpn connect "My UPV Connection"
upv vpn disconnect
upv vpn delete "My UPV Connection"
upv vpn delete "UPV Work" --force  # Skip confirmation
upv vpn list
upv vpn purge                       # Delete all UPV connections (with double confirmation)
upv vpn purge --force              # Delete all UPV connections without confirmation
upv vpn purge --except "Keep This" # Delete all except specified connections
upv vpn purge -e "VPN1" -e "VPN2"  # Delete all except VPN1 and VPN2
upv vpn status
upv drive mount myuser UPVNET --drive W --open  # Uses VPN credentials
upv drive mount myuser UPVNET --password mypass --drive W --open  # Uses explicit credentials
upv drive mount myuser ALUMNO -d W -o  # Short flags, uses VPN credentials
upv drive mount myuser ALUMNO -p mypass -d W -o  # Short flags with password
upv drive unmount --drive W
upv drive status
upv completions powershell # Generate PowerShell shell completions script
upv completions powershell > upv-completions.ps1 # Save PowerShell shell completions script to a file
```

Use `--help` to see available options:

```bash
upv --help
upv vpn --help
upv drive --help
```

---

## 🧩 Shell Completions

`upv-cli` supports shell auto-completions for various shells thanks to the `upv completions` command.

In the following instructions, we focus on **PowerShell**, but you can adapt them for other shells like Bash or Zsh.

As of writing, the `clap_complete` library supports the following shells:
- Bash
- Elvish
- Fish
- PowerShell
- Zsh

For more information, check out the [clap_complete library documentation](https://docs.rs/clap_complete/latest/clap_complete/).

> If they add more shells in the future, feel free to open an issue or PR to update `clap_complete`'s version in this project, and also update the README including those shells in the list.

You have two easy ways to enable completions:

### 1. Dynamic completion loading (recommended for ease of use)

Add this line to your PowerShell profile (`$PROFILE`), so completions load dynamically every time you open a new shell:

```powershell
Invoke-Expression (& upv completions powershell | Out-String)
```

If you want to make it so it only runs when `upv` is available, you can add this instead:

```powershell
if (Get-Command upv -ErrorAction SilentlyContinue) {
    Invoke-Expression (& upv completions powershell | Out-String)
}
```

This runs the completions command at startup and loads the completions script in memory, always matching the current version of the tool. It adds a tiny bit of startup overhead but requires zero manual updates.

### 2. Static completions file

If you prefer faster shell startup and don't want to run the completion generator every time:

1. Generate the completions file and save it somewhere (e.g., `~\upv-completions.ps1`):

```powershell
upv completions powershell > $HOME\upv-completions.ps1
```

2. Add this line to your PowerShell profile (`$PROFILE`) to load the saved completions script:

```powershell
. $HOME\upv-completions.ps1
```

Now your completions load instantly, but you'll need to regenerate the file manually if you update `upv`.

---

## 🚪 Exit codes

The `upv` command returns the following exit codes:

- `0`: Success

### General errors:

These errors are common to all commands and indicate general issues:

- `1`: General error (the program failed)
- `2`: Invalid command or argument (parsing error)

### Specific upv-cli errors:

These errors are specific to the `upv-cli` tool and typically indicate issues with VPN or drive operations:

- `10`: Generic upv-cli error
- `11`: VPN error
- `12`: Drive error
- `13`: Drive in use error (files or folders are open on the drive)

---

## 🛠️ Development

Clone the repo and run locally:

```bash
git clone https://github.com/algono/upv-cli
cd upv-cli
cargo run -- <your-args>
```

## How to publish a new version

> **Note:** Requires Rust and Cargo installed. If you don't have them, follow the [Rust installation guide](https://www.rust-lang.org/tools/install).

If you want to create a new version, remember to update the version in `Cargo.toml` before building.

#### For Crates\.io

Publish the new version to [crates.io](https://crates.io/crates/upv-cli):

```bash
cargo publish
```

#### For GitHub Releases + Scoop

- Build the project in release mode:

```bash
cargo build --release
```

- Generate hash for the latest release:

```pwsh
$hash = (Get-FileHash -Path .\target\release\upv.exe -Algorithm SHA256).Hash.ToLower()
$hash > .\target\release\upv.exe.sha256
```

Then, upload both the `upv.exe` and `upv.exe.sha256` files from `target/release/` to the GitHub Releases page (for this repo, that would be at: <https://github.com/algono/upv-cli/releases/new>).

##### Scoop bucket update

If you own any Scoop bucket targeting this tool (such as [scoop-algono](https://github.com/algono/scoop-algono)), the update the `upv-cli.json` manifest must be updated.

That could happen automatically if you have a GitHub Action that updates the manifest (such as `Excavate` from the Scoop Bucket Template) running on a schedule. You can also manually trigger the GitHub Action to update the manifest.

> Note that the Excavate GitHub Action will only work if the manifest is properly configured with `checkver` and `autoupdate` fields.

You can also update the manifest manually, by changing the `version`, `url` and `hash` fields in the `upv-cli.json` manifest file, and then committing and pushing the changes to your bucket repository.

This is an example of the `upv-cli.json` manifest file, extracted from the [scoop-algono](https://github.com/algono/scoop-algono) bucket:

```json
{
    "version": "0.4.0",
    "description": "A CLI tool for managing VPN and network shares from UPV (Universitat Politècnica de València) on Windows.",
    "homepage": "https://github.com/algono/upv-cli",
    "license": "MIT OR Apache-2.0",
    "url": "https://github.com/algono/upv-cli/releases/download/v0.4.0/upv.exe",
    "hash": "cc800b5f6a67328581010670b54aa9c0857d7f0b9faec0b3fec47c7162620b9b",
    "bin": "upv.exe",
    "checkver": "github",
    "autoupdate": {
        "url": "https://github.com/algono/upv-cli/releases/download/v$version/upv.exe",
        "hash": {
            "url": "$url.sha256"
        }
    }
}
```

---

## 🧾 License

Licensed under either of:

* [MIT License](LICENSE-MIT)
* [Apache License, Version 2.0](LICENSE-APACHE)

at your option.

---

## 🙋 About

This tool was developed by [Alejandro Gómez (algono)](https://github.com/algono) to simplify VPN and network drive access for students and staff at [UPV](https://www.upv.es/index-en.html).

---

#### 🗃️ Older versions

> ⚠️ Notice: The tools mentioned here are no longer maintained, and may contain some outdated features

For a GUI solution, check out: [AccesoUPV](https://github.com/algono/AccesoUPV)
