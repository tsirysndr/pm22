use crate::pm2::run_pm2_command;
use clap::{Arg, Command as ClapCommand};
use owo_colors::OwoColorize;
use std::env;

pub mod pm2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let banner = format!(
        "{}\nConnects to a remote server via SSH and executes PM2 commands",
        r#"
    ██████  ███    ███ ██████  ██████
    ██   ██ ████  ████      ██      ██
    ██████  ██ ████ ██  █████   █████
    ██      ██  ██  ██ ██      ██
    ██      ██      ██ ███████ ███████
"#
        .cyan()
    );
    let matches = ClapCommand::new("pm22")
        .about(&banner)
        .version(env!("CARGO_PKG_VERSION"))
        .author("Tsiry Sandratraina <tsiry.sndr@rocksky.app>")
        .arg(
            Arg::new("host")
                .short('h')
                .long("host")
                .default_value("PM22_HOST")
                .help("Host to connect to, with username (e.g., user@host)"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .default_value("22")
                .help("Port to connect to (default: 22)"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Enable verbose output"),
        )
        .arg(
            Arg::new("key")
                .short('k')
                .long("key")
                .default_value("~/.ssh/id_rsa")
                .help("SSH private key file (default: ~/.ssh/id_rsa)"),
        )
        .arg(
            Arg::new("cmd")
                .required(true)
                .help("PM2 Command to run on the remote server, e.g., ps, status, start, stop, restart, delete, logs, etc.")
        )
        .arg(
            Arg::new("args")
                .trailing_var_arg(true)
                .allow_hyphen_values(true)
                .num_args(0..)
                .help("Arguments to pass to pm2 command"),
        )
        .get_matches();

    let verbose = matches.get_flag("verbose");
    init_logger(verbose);

    let host = env::var("PM22_HOST")
        .unwrap_or_else(|_| matches.get_one::<String>("host").unwrap().to_string());
    let host = match host.as_str() {
        "PM22_HOST" => env::var("PM22_HOST")?,
        _ => host,
    };
    let port = env::var("PM22_PORT")
        .unwrap_or_else(|_| matches.get_one::<String>("port").unwrap().to_string());
    matches.get_one::<String>("port").unwrap();
    let key = env::var("PM22_KEY")
        .unwrap_or_else(|_| matches.get_one::<String>("key").unwrap().to_string());
    let cmd = matches.get_one::<String>("cmd").unwrap();
    let args: Vec<&String> = matches
        .get_many::<String>("args")
        .map(|vals| vals.collect())
        .unwrap_or_default();

    run_pm2_command(
        host.trim(),
        port.parse().unwrap_or(22),
        key.trim(),
        cmd,
        args,
    )?;
    Ok(())
}

pub fn init_logger(verbose: bool) {
    let level = if verbose { "debug" } else { "warn" };
    unsafe { std::env::set_var("RUST_LOG", level) };
    env_logger::init();
}
