use crate::config::read_config_suite;
use crate::discord::{form_url, get};
use anyhow::Result;
use spectacles_model::User;

pub fn run() {
    let config = read_config_suite();

    let user: Result<User, _> = get(form_url("/users/@me"), config.token);
    if user.is_err() {
        return println!("An unknown error occurred when trying to fetch the user");
    }
    let user = user.unwrap();
    println!("{:#?}", user);
}
