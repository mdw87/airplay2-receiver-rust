use clap::Parser;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(name = "airplay2-receiver")]
#[command(author = "AirPlay2 Receiver Contributors")]
#[command(version = "0.1.0")]
#[command(about = "A high-performance AirPlay 2 audio receiver", long_about = None)]
struct Args {
    /// Device name visible to AirPlay clients
    #[arg(short, long, default_value = "AirPlay Receiver")]
    name: String,

    /// Network interface to bind to (e.g., "wlan0" or "192.168.1.100")
    #[arg(short, long)]
    interface: Option<String>,

    /// Server port (default: 7000 - standard AirPlay port)
    #[arg(short, long, default_value_t = 7000)]
    port: u16,

    /// Disable volume control
    #[arg(long)]
    no_volume: bool,

    /// Enable verbose logging (debug level)
    #[arg(short, long)]
    verbose: bool,

    /// Enable trace-level logging (very verbose)
    #[arg(long)]
    trace: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Set up logging based on verbosity flags
    let log_level = if args.trace {
        Level::TRACE
    } else if args.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set tracing subscriber");

    info!("Starting AirPlay 2 Receiver");
    info!("Device name: {}", args.name);
    info!("Port: {}", args.port);

    if let Some(ref iface) = args.interface {
        info!("Interface: {}", iface);
    }

    if args.no_volume {
        info!("Volume control: disabled");
    }

    // TODO: Initialize the receiver
    info!("Receiver initialization not yet implemented");
    info!("Run 'cargo build' to verify the project structure is set up correctly");

    Ok(())
}
