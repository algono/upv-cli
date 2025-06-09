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

> ⚠️ **Note:** This tool is only available for **Windows**, since it uses Windows-specific features like `net use` and `rasdial`.

Install from [crates.io](https://crates.io/crates/upv-cli):

```bash
cargo install upv-cli
```

Or from source:

```bash
cargo install --path .
```

Make sure `~/.cargo/bin` is in your `PATH`.

> 📝 **Note:** After installing, the binary is available as `upv`.

---

## ⚡ Quickstart

```bash
cargo install upv-cli
upv vpn create UPV --connect
upv drive mount UPVNET -u myuser -d W -o
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
