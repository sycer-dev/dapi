use clap::ArgMatches;
use serde_json::json;

pub fn run(matches: &ArgMatches) {
    match matches.subcommand() {
        ("message", Some(matches)) => {
            build_message(matches);
        }
        _ => unreachable!(),
    }
}

fn build_message(matches: &ArgMatches) {
    let content = matches.value_of("content");
    let json = json!({ "content": content });
    println!("{}", json.to_string());
}
