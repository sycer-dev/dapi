use crate::config::read_config_suite;
use crate::discord::{delete, form_url, get, patch, post, put};
use clap::ArgMatches;
use serde_json::Value;
use termion::{color, style};

fn to_pretty(data: Value) -> String {
	return serde_json::to_string_pretty(&data).unwrap();
}

pub fn run(matches: &ArgMatches) {
	let method = matches.value_of("method").unwrap();
	let route = form_url(matches.value_of("route").unwrap());
	let data = matches.value_of("data");
	let info = format!(
		"{}{}{}{}{} @ {}{}{}",
		style::Bold,
		color::Fg(color::LightGreen),
		method.to_uppercase(),
		color::Fg(color::Reset),
		style::Reset,
		color::Fg(color::LightBlue),
		route,
		color::Fg(color::Reset)
	);

	let config = read_config_suite();
	if method == "get" {
		let res = get::<Value>(route.to_string(), config.token);
		return match res {
			Ok(data) => {
				println!("{}\n{}", info, to_pretty(data));
			}
			Err(e) => {
				println!("{}\n{:#?}", info, e);
			}
		};
	}

	let mut input = String::new();
	if data.is_some() && input.len() == 0 {
		input = data.unwrap().to_string();
	} else {
		let _ = std::io::stdin().read_line(&mut input);
	}

	println!(
		"{}Recieved input{}: {}",
		color::Fg(color::LightCyan),
		color::Fg(color::Reset),
		input
	);

	let res = match method {
		"patch" => patch::<Value>(route.to_string(), input, config.token),
		"put" => put::<Value>(route.to_string(), input, config.token),
		"post" => post::<Value>(route.to_string(), input, config.token),
		"delete" => delete::<Value>(route.to_string(), input, config.token),
		_ => unreachable!(),
	};

	return match res {
		Ok(data) => {
			if data.get("code").is_some() {
				println!(
					"{}\n{}{}{}{}{}\n{}",
					info,
					style::Bold,
					color::Fg(color::LightRed),
					"API Error",
					color::Fg(color::Reset),
					style::Reset,
					to_pretty(data),
				);
			} else {
				println!("{}\n{}", info, to_pretty(data));
			}
		}
		Err(e) => {
			println!(
				"{}\n {}{}{}{}{} {:#?}",
				info,
				style::Bold,
				color::Fg(color::LightRed),
				"An error occured:",
				color::Fg(color::Reset),
				style::Reset,
				e
			);
		}
	};
}
