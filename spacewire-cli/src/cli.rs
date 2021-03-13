use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Command {
    /// Operate a relay
    Relay(RelayOpts),
    /// Send a file or a directory
    Send(SendOpts),
}

#[derive(StructOpt, Debug)]
pub struct RelayOpts {
    /// The address the relay will listen on
    #[structopt(default_value = ":::6200")]
    pub address: String,
}

#[derive(StructOpt, Debug)]
pub struct SendOpts {}

#[derive(StructOpt, Debug)]
pub struct Opts {
    #[structopt(subcommand)]
    pub command: Command,
}
