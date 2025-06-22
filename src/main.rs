use crate::pm2::run_pm2_command;
use clap::{Arg, Command as ClapCommand};

pub mod pm2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = ClapCommand::new("pm22")
        .about("Connects to a remote server via SSH and executes PM2 commands")
        .version("0.1.0")
        .author("Tsiry Sandratraina <tsiry.sndr@rocksky.app>")
        .arg(
            Arg::new("host")
                .required(true)
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

    let host = matches.get_one::<String>("host").unwrap();
    let port = matches.get_one::<String>("port").unwrap();
    let key = matches.get_one::<String>("key").unwrap();
    let cmd = matches.get_one::<String>("cmd").unwrap();
    let args: Vec<&String> = matches
        .get_many::<String>("args")
        .map(|vals| vals.collect())
        .unwrap_or_default();

    run_pm2_command(host, port.parse().unwrap_or(22), key, cmd, args)?;
    Ok(())
}

pub fn init_logger(verbose: bool) {
    let level = if verbose { "debug" } else { "warn" };
    unsafe { std::env::set_var("RUST_LOG", level) };
    env_logger::init();
}
