use bech32::ToBase32;
use log::{debug, trace};
use structopt::StructOpt;

mod cli;

use spacewire::link::EphemeralLink;
use spacewire::relay::Relay;

/// Encodes the given `key` into the improved Bech32 (BIP 0350) format.
fn encode_public_key<T: AsRef<[u8]>>(key: T) -> String {
    bech32::encode("SPC", key.as_ref().to_base32(), bech32::Variant::Bech32m).unwrap()
}

async fn cmd_send(_opts: &cli::Opts, send_opts: &cli::SendOpts) -> Result<(), anyhow::Error> {
    let relay = "localhost:6200";

    trace!("Starting send with options: {:?}", send_opts);
    trace!("Creating new ephemeral link");
    let ephemeral_link = EphemeralLink::new()?;

    let public_key = ephemeral_link.public_key()?;
    debug!(
        "Created ephemeral link with public key {}",
        encode_public_key(public_key)
    );

    trace!("Initiating connection to relay tcp://{}", relay);
    ephemeral_link.connect(relay).await?;

    Ok(())
}

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
        Command::Send(ref send_opts) => {
            cmd_send(&opts, send_opts).await?;
        }
        _ => {
            println!("Hello, world!");
        }
    }

    Ok(())
}
