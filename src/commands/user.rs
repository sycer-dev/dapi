use crate::config::read_config_suite;
use crate::discord::user;
use clap::ArgMatches;

pub fn run(matches: &ArgMatches) {
	let config = read_config_suite();
	let id = matches.value_of("id");
	let user = user(config.token, id.unwrap().into());
	println!("{:#?}", user);
}
