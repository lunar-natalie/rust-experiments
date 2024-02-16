use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct VNC {
    id: u32,
	ip: String,
	port: u32,
	city: String,
	state: String,
	country: String,
	clientname: String,
	screenres: String,
	hostname: String,
	osname: String,
	openports: String,
	username: String,
	password: String,
	createdat: u32
}

fn search(endpoint: String, client_name: String) -> Vec<VNC> {
    return vec![];
}