pub mod args;
mod http;
mod lnd;
mod zf;
pub mod scheduler;

use bitcoin::Amount;
use scheduler::Scheduler;

use crate::args::parse_args;

#[macro_use]
extern crate serde_derive;
extern crate configure_me;

configure_me::include_config!();

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let (config, args) =
    //     Config::including_optional_config_files(std::iter::empty::<&str>()).unwrap_or_exit();

    // let scheduled_pj = parse_args(args).expect("failed to parse remaining arguments");
    // let secure_endpoint: url::Url = config.endpoint.parse().expect("Malformed secure endpoint from config file. Expecting a https or .onion URI to proxy payjoin requests");

    // let scheduler = Scheduler::from_config(&config).await?;

    // if let Some(payjoin) = scheduled_pj {
    //     let address = scheduler.schedule_payjoin(&payjoin).await?;
    //     println!("{}", scheduler::format_bip21(
    //         address,
    //         payjoin.total_amount(),
    //         secure_endpoint.clone()
    //     ));
    // }

    // let bind_addr = ([127, 0, 0, 1], config.bind_port).into();
    // http::serve(scheduler, bind_addr, secure_endpoint).await?;
    let amount: Amount = Amount::from_sat(100_000);
    zf::schedule_inbound_openings("038b177029ed7aa196cf038e1e264be2c35e02e1397d5975ac6a329cc7cdd3a1e6@rxc7r65xzbb5mz6cf6mtgduiu54ht2yk4r7q6yl6gk7xfpqd2jm6u4id.onion:9735".to_string(), amount).await;

    Ok(())
}
