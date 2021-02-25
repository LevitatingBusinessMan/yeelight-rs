use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::io::Error;
use std::io;
use regex::Regex;
use std::net::TcpStream;
use std::io::{Write,Read};
use serde::{Deserialize, Serialize};

pub struct Light {

	pub headers: HashMap<String,String>,
	pub ip: Ipv4Addr,
	pub port: u16,
	pub socket: TcpStream

}

#[derive(Serialize, Deserialize)]
struct Command {
	id: u32,
	method: String,
	params: Vec<Param>
}

#[derive(Serialize, Deserialize)]
//I would call it "Result" but that would interfere.
struct Response {
	id: u32,
	result: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct Notification {

}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Param {
	Int(u32),
	String(String)
}

/**
 * Gotta start thinking about how to handle async use here.
 * Can the result include some kind of promise? Futures?
 * Will every light get its own thread to listen for calls? Also future crate?
 */

impl Light {
	pub fn new(headers: HashMap<String,String>) -> Result<Light, Error> {
		
		if !headers.contains_key("Location") {
			return Err(Error::new(io::ErrorKind::InvalidInput, "Missing location header"));
		}

		let re = Regex::new(r"\w://([\d.]+):(\d+)").unwrap();
		let captures = re.captures(headers.get("Location").unwrap()).unwrap();
		
		let ip = captures.get(1).unwrap().as_str().parse::<Ipv4Addr>().unwrap();
		let port = captures.get(2).unwrap().as_str().parse::<u16>().unwrap();

		//let ip = Ipv4Addr::new(127, 0, 0, 1);
		//let port = 3333;

		let socket = TcpStream::connect((ip, port)).expect("Failed connecting to the server");

		Ok(Light{headers, ip, port, socket})
	}

	pub fn send_command(&mut self, method: &str, params: Vec<Param>) -> Result<(), Error> {
		
		let command = Command {
			id: 0,
			method: method.to_owned(),
			params
		};

		let mut payload = serde_json::to_string(&command).unwrap();

		payload.push_str("\r\n");
		let payload = payload;

		println!("{}", payload);

		self.socket.write(payload.as_bytes());
		
		let mut buf = [0; 128];
		self.socket.read(&mut buf)?;
		println!("{}",std::str::from_utf8(&buf).unwrap());


		Ok(())
	}

	pub fn toggle(&mut self) -> Result<(), Error> {
		self.send_command("toggle", vec![])
	}

	pub fn set_bright(&mut self, brightness: u32, effect: &str, duration: u32) -> Result<(), Error> {
		self.send_command("set_bright", vec![Param::Int(brightness), Param::String(effect.to_owned()), Param::Int(duration)])
	}
 
}
