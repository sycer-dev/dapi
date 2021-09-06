use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use spectacles_model::{message::Message, User};

const ENDPOINT: &'static str = "https://discord.com/api/v9";

pub fn form_url(path: &str) -> String {
	return format!("{}{}", ENDPOINT, &path);
}

pub fn validate_token(token: String) -> Result<(), String> {
	let me: Result<User, _> = get(form_url("/users/@me"), token);
	match me {
		Ok(_) => Ok(()),
		_ => Err(String::from("Please provide a valid Discord bot token!")),
	}
}

pub fn deob_token(token: &str) -> String {
	let split: Vec<&str> = token.split(".").collect();
	return format!("{}.{}.{}", split[0], censor(split[1]), censor(split[2]));
}

pub fn censor(content: &str) -> String {
	return "*".repeat(content.len());
}

pub fn get<T: Serialize + for<'de> Deserialize<'de>>(url: String, token: String) -> Result<T> {
	let data = Client::new()
		.get(&url)
		.header("Authorization", format!("Bot {}", token))
		.send()
		.expect("Request failed")
		.json()
		.expect("Failed to parse the json");
	Ok(data)
}

pub fn post<T: Serialize + for<'de> Deserialize<'de>>(
	url: String,
	body: String,
	token: String,
) -> Result<T> {
	let data = Client::new()
		.post(&url)
		.header("Authorization", format!("Bot {}", token))
		.header("Content-Type", "application/json")
		.body(body)
		.send()
		.expect("Request failed")
		.json()
		.expect("Failed to parse the json");
	Ok(data)
}

pub fn patch<T: Serialize + for<'de> Deserialize<'de>>(
	url: String,
	body: String,
	token: String,
) -> Result<T> {
	let data = Client::new()
		.patch(&url)
		.header("Authorization", format!("Bot {}", token))
		.header("Content-Type", "application/json")
		.body(body)
		.send()
		.expect("Request failed")
		.json()
		.expect("Failed to parse the json");
	Ok(data)
}

pub fn delete<T: Serialize + for<'de> Deserialize<'de>>(
	url: String,
	body: String,
	token: String,
) -> Result<T> {
	let data = Client::new()
		.delete(&url)
		.header("Authorization", format!("Bot {}", token))
		.header("Content-Type", "application/json")
		.body(body)
		.send()
		.expect("Request failed")
		.json()
		.expect("Failed to parse the json");
	Ok(data)
}

pub fn put<T: Serialize + for<'de> Deserialize<'de>>(
	url: String,
	body: String,
	token: String,
) -> Result<T> {
	let data = Client::new()
		.put(&url)
		.header("Authorization", format!("Bot {}", token))
		.header("Content-Type", "application/json")
		.body(body)
		.send()
		.expect("Request failed")
		.json()
		.expect("Failed to parse the json");
	Ok(data)
}

pub fn user(token: String, id: String) -> User {
	return get::<User>(form_url(&format!("/users/{}", id)), token).unwrap();
}

#[allow(dead_code)]
pub fn me(token: String) -> User {
	return user(token, "@me".into());
}

pub fn create_message(token: String, channel_id: String, body: String) -> Message {
	return post::<Message>(
		form_url(&format!("/channels/{}/messages", channel_id)),
		body,
		token,
	)
	.unwrap();
}
