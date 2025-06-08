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
```

Use `--help` to see available options:

```bash
upv --help
upv vpn --help
upv drive --help
```

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
