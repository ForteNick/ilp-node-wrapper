#[macro_use]
extern crate clap;
use clap::{App, Arg};
use interledger::node::InterledgerNode;

#[allow(clippy::cognitive_complexity)]
fn main() {
    env_logger::init();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Interledger Node Wrapper")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .required(true)
                .help("Service config file"),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap();
    println!("Value for config_path: {}", config_path);

    let mut node_config = config::Config::new();
    node_config
        .merge(config::File::with_name(config_path))
        .expect("Error reading config file");

    let node: InterledgerNode = node_config
        .try_into()
        .expect("Must provide config file name or config environment variables");
    node.run();
}
