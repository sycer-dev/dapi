use crate::config::read_config_suite;
use crate::discord::create_message;
use clap::ArgMatches;

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        ("post", Some(matches)) => {
            send_message(matches);
        }
        _ => unreachable!(),
    }
}

fn send_message(matches: &ArgMatches) {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input);
    let config = read_config_suite();
    let id = matches.value_of("channel_id");
    let msg = create_message(config.token, id.unwrap().into(), input);
    println!("{:#?}", msg);
}
