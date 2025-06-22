use std::{io::Read, net::TcpStream, path::Path};

use anyhow::Error;
use base64::{engine::general_purpose, Engine};
use log::info;
use owo_colors::OwoColorize;
use regex::Regex;
use ssh2::Session;

pub fn run_pm2_command(
    host: &str,
    port: u32,
    key: &str,
    cmd: &str,
    args: Vec<&String>,
) -> Result<(), Error> {
    let parts: Vec<&str> = host.split('@').collect();
    if parts.len() != 2 {
        return Err(Error::msg("Host must be in the format user@host"));
    }
    let username = parts[0];

    let host = parts[1];

    let ssh_url = format!("{}@{}:{}", username, host, port);
    info!(
        "Connecting to {} using key {} ...",
        ssh_url.bright_cyan(),
        key.bright_cyan()
    );

    let tcp = TcpStream::connect(format!("{}:{}", host, port))?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    let private_key_path = shellexpand::tilde(key).to_string();
    let private_key = Path::new(&private_key_path);
    session.userauth_pubkey_file(username, None, private_key, None)?;
    if !session.authenticated() {
        return Err(Error::msg("SSH authentication failed"));
    }

    info!("Connected to {} successfully!", ssh_url.bright_cyan());

    let mut channel = session.channel_session()?;
    let command = format!(
        "NO_NEOFETCH=1 bash -lic 'pm2 {} {}'",
        cmd,
        args.iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(" ")
    );

    info!("Executing command: {}", command.bright_yellow());

    channel.request_pty("xterm", None, None)?;

    setup_pm2(&session)?;

    channel.exec(&command)?;

    let mut buffer = [0; 1024];

    // Stream stdout in real-time
    loop {
        match channel.read(&mut buffer) {
            Ok(0) => break, // EOF, command finished
            Ok(n) => {
                let output = String::from_utf8_lossy(&buffer[..n]);
                let clean_output = clean_terminal_noises(&output);
                print!("{}", clean_output); // Real-time output
            }
            Err(e) => {
                eprintln!("Error reading from channel: {}", e);
                break;
            }
        }
    }

    channel.wait_close()?;
    info!("Exit status: {}", channel.exit_status()?);

    Ok(())
}

pub fn setup_pm2(session: &Session) -> Result<(), Error> {
    let mut channel = session.channel_session()?;
    channel.request_pty("xterm", None, None)?;

    info!("Setting up PM2 on the remote server...");

    let install_script = r#"#!/bin/bash
    set -e

    if ! command -v pm2 &> /dev/null; then
        echo "PM2 not found, installing..."
        curl https://mise.run | sh
        export PATH=$HOME/.local/bin:$PATH
        export PATH=$HOME/.local/share/mise/shims:$PATH
        echo 'export PATH=$HOME/.local/bin:$PATH' >> ~/.bashrc
        echo 'export PATH=$HOME/.local/share/mise/shims:$PATH' >> ~/.bashrc
        mise install node
        mise use -g node
        npm install -g pm2
    fi
    "#;

    let encoded_script = general_purpose::STANDARD.encode(install_script);

    let remote_command = format!(
        "echo {} | base64 -d > /tmp/install-pm2.sh && chmod +x /tmp/install-pm2.sh && /tmp/install-pm2.sh",
        encoded_script
    );

    channel.exec(&format!("NO_NEOFETCH=1 bash -lic '{}'", remote_command))?;

    let mut buffer = [0; 1024];
    loop {
        match channel.read(&mut buffer) {
            Ok(0) => break, // EOF, script finished
            Ok(n) => {
                let output = String::from_utf8_lossy(&buffer[..n]);
                let clean_output = clean_terminal_noises(&output);
                print!("{}", clean_output); // Real-time output
            }
            Err(e) => {
                eprintln!("Error reading from channel: {}", e);
                break;
            }
        }
    }

    info!("PM2 setup completed successfully!");

    Ok(())
}

fn clean_terminal_noises(s: &str) -> String {
    let osc_re = Regex::new(r"\x1b\][^\x07\x1b]*(\x07|\x1b\\)").unwrap();
    let csi_dsr_re = Regex::new(r"\x1b\[\d+;\d+R").unwrap();
    let malformed_csi_re = Regex::new(r"\[{1,2}\d{1,3}(;\d{1,3})?R").unwrap();
    let cleaned = osc_re.replace_all(s, "");
    let cleaned = csi_dsr_re.replace_all(&cleaned, "");
    let cleaned = malformed_csi_re.replace_all(&cleaned, "");
    cleaned.into_owned()
}
