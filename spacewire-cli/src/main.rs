use structopt::StructOpt;

mod cli;

use spacewire::relay::Relay;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    use cli::Command;

    pretty_env_logger::init_timed();

    // Parse the command-line arguments
    let opts = cli::Opts::from_args();

    match &opts.command {
        Command::Relay(ref relay_opts) => {
            let relay = Relay::bind(&relay_opts.address).await?;

            relay.poll().await?;
        }
        _ => {
            println!("Hello, world!");
        }
    }

    Ok(())
}
