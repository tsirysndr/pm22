# PM22

[![downloads](https://img.shields.io/crates/dr/pm22)](https://crates.io/crates/pm22)
[![crates](https://img.shields.io/crates/v/pm22.svg)](https://crates.io/crates/pm22)



`pm22` is a lightweight CLI tool that connects to a remote server over SSH and executes [PM2](https://pm2.keymetrics.io/) process manager commands.

Perfect for remotely managing Node.js applications from your terminal.

![Preview](https://raw.githubusercontent.com/tsirysndr/pm22/main/.github/assets/preview.png)


## ✨ Features

- SSH into any server with your private key
- Auto install Node.js and PM2 if not found
- Execute any PM2 command remotely (`start`, `stop`, `restart`, `delete`, `logs`, etc.)
- Supports custom ports and SSH keys
- Optional verbose output with `--verbose`

## 🚚 Installation
You can install `pm22` globally using cargo:

```bash
cargo install pm22
```

## 🚀 Usage

```bash
pm22 [OPTIONS] <cmd> [args]...
```

## 🔹 Arguments

| Name        |	Description                                                         |
| ----------- | ------------------------------------------------------------------- |
| `<cmd>`	    | PM2 command to execute (start, restart, status, logs, etc.)         |
| `[args]...` |	Additional arguments passed to the PM2 command                      |

## 🔹 Options

| Flag	                | Description	                                    | Default         |
| --------------------- | ----------------------------------------------- | --------------- |
| `-h`, `--host <host>`	| Host to connect to (e.g., user@your.server.com) |                 |
| `-p`, `--port <port>`	| SSH port to connect to	                        | `22`            |
| `-k`, `--key <path>`	| Path to your SSH private key                    | `~/.ssh/id_rsa` |
| `-v`, `--verbose`	    | Enable verbose output for debugging/logging     |                 |
| `--help`	            | Show help information	                          |                 |
| `-V`, `--version`	    | Show version information                        |                 |

## 📄 License
MIT License © 2025 [Tsiry Sandratraina](https://github.com/tsirysndr)