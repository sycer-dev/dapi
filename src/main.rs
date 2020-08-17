#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate clap;

use crate::discord::validate_token;
use clap::{App, Arg};

mod commands;
mod config;
mod discord;
mod util;

fn main() {
    let _ = util::check_version();
    let version = crate_version!();

    let matches =
        App::new("dapi")
            .about("a cli app to interact with the discord api")
            .version(version)
            .author(crate_authors!())
            .subcommand(
                App::new("build")
                    .about("builds data for piping into post/put/patch requests")
                    .subcommand(App::new("message").arg(Arg::from_usage(
                        "-c, --content=[CONTENT], The provided message content",
                    ))),
            )
            .subcommand(
                App::new("config")
                    .about("manages the configuration")
                    .subcommand(App::new("destroy").about("destroys the configuration file"))
                    .subcommand(App::new("init").about("creates a configuration file"))
                    .subcommand(
                        App::new("set")
                            .about("sets a value in the config")
                            .subcommand(
                                App::new("token").arg(
                                    Arg::with_name("token")
                                        .required(true)
                                        .validator(validate_token),
                                ),
                            ),
                    ),
            )
            .subcommand(
                App::new("messages")
                    .aliases(&["message", "msg"])
                    .about("reads/writes messages to/from the discord api")
                    .subcommand(App::new("post").about("creates a new message").arg(
                        Arg::from_usage(
                            "<channel_id>, the id of the channel to send this message to",
                        ),
                    )),
            )
            .subcommand(
                App::new("user")
                    .about("fetches a user from the discord api")
                    .arg(Arg::with_name("id").required(true)),
            )
            .subcommand(App::new("whoami").about("returns info on the configured Discord token"))
            .get_matches();

    match matches.subcommand() {
        ("build", Some(matches)) => {
            commands::build::run(matches);
        }
        ("config", Some(matches)) => {
            commands::config::run(matches);
        }
        ("messages", Some(matches)) => {
            commands::messages::run(matches);
        }
        ("user", Some(matches)) => {
            commands::user::run(matches);
        }
        ("whoami", _) => {
            commands::whoami::run();
        }
        _ => unreachable!(),
    }
}
