use std::{io::Read, net::TcpStream, path::Path};

use anyhow::Error;
use log::info;
use owo_colors::OwoColorize;
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
    channel.exec(&command)?;

    let mut buffer = [0; 1024];

    // Stream stdout in real-time
    loop {
        match channel.read(&mut buffer) {
            Ok(0) => break, // EOF, command finished
            Ok(n) => {
                let output = String::from_utf8_lossy(&buffer[..n]);
                print!("{}", output); // Real-time output
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
