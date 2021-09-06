use home::home_dir;
use std::fs;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
	pub token: String,
}

pub fn config_dir() -> String {
	let home = match home_dir() {
		Some(dir) => dir.to_str().unwrap().to_owned(),
		_ => "".into(),
	};
	return format!("{}/{}", home, ".dapi");
}

pub fn config_path() -> String {
	return format!("{}/{}", config_dir(), "config.toml");
}

pub fn read_config() -> Option<Config> {
	let home = match home_dir() {
		Some(dir) => dir.to_str().unwrap().to_owned(),
		_ => "".into(),
	};

	let path = format!("{}/{}/{}", home, ".dapi", "config.toml");
	let file = fs::read(path);
	match file {
		Ok(file) => {
			let parsed = toml::from_slice::<Config>(&file);
			match parsed {
				Ok(config) => Some(config),
				_ => None,
			}
		}
		_ => None,
	}
}

pub fn read_config_suite() -> Config {
	let config = read_config();
	if config.is_none() {
		println!("There is no configured Discord bot token! Please set one with 'dapi config set token token_here'");
		std::process::exit(0);
	}
	return config.unwrap();
}

pub fn create_config(content: &str) -> Result<String, String> {
	let dir_path = config_dir();
	let _ = fs::create_dir(&dir_path);

	let file_path = config_path();
	let file = fs::write(&file_path, content);
	match file {
		Ok(_) => Ok(format!(
			"Successfully created the configuration file at {}",
			file_path
		)),
		Err(err) => Err(format!(
			"An error occurred when trying to create the configuration file: {}",
			err
		)),
	}
}
