// mod cli;
// mod client;
mod crypto;
mod sdk;
mod display;
mod proto;
mod utils;
// mod wallet;
mod cmd;
mod config;
mod interactive;

use config::Config;
use cmd::all_cmd;
use crypto::EthCrypto;

fn main() {
    let config = Config {
        controller_addr: "localhost:50005".into(),
        executor_addr: "localhost:50002".into(),
        default_account: None,
        wallet_dir: "d:/cld/cloud-cli/tmp-wallet".into(),
    };

    let mut ctx = sdk::context::from_config::<EthCrypto>(&config).unwrap();
    let cmd = all_cmd();

    let matches = cmd.get_matches();
    cmd.exec(&mut ctx, matches).unwrap();
}
