use bech32::ToBase32;
use log::trace;
use structopt::StructOpt;

mod cli;

use spacewire::Identity;
use spacewire::{Link, Relay};

/// Encodes the given `key` into the improved Bech32 (BIP 0350) format.
fn encode_public_key<T: AsRef<[u8]>>(key: T) -> String {
    bech32::encode("SPC", key.as_ref().to_base32(), bech32::Variant::Bech32m).unwrap()
}

async fn cmd_send(_opts: &cli::Opts, send_opts: &cli::SendOpts) -> Result<(), anyhow::Error> {
    let relay = "localhost:6200";

    trace!("Starting send with options: {:?}", send_opts);
    trace!("Generating a new identity");

    let identity = Identity::new()?;

    trace!(
        "Created identity with public key {}",
        encode_public_key(identity.public_key())
    );

    trace!("Creating new link with associated identity");
    let link = Link::new_with_identity(relay, identity)?;

    trace!("Initiating connection to relay tcp://{}", relay);
    link.connect().await?;

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
    }

    Ok(())
}
