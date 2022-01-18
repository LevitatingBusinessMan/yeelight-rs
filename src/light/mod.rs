use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::io::Error;
use std::io;
use regex::Regex;
use std::net::TcpStream;
use std::io::{Write,Read};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Light {

	pub headers: HashMap<String,String>,
	pub ip: Ipv4Addr,
	pub port: u16,
	pub socket: TcpStream,
	last_id: u32

}

#[derive(Serialize, Deserialize)]
struct Command {
	id: u32,
	method: String,
	params: Vec<Param>
}

#[derive(Serialize, Deserialize, Debug)]
//I would call it "Result" but that would interfere.
pub struct Response {
	id: u32,
	result: Vec<String>
}

#[derive(Serialize, Deserialize)]
struct Notification {

}

#[derive(Serialize, Deserialize, Clone)]
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

		let last_id = 999;

		Ok(Light{headers, ip, port, socket, last_id})
	}

	pub fn send_command(&mut self, method: &str, params: Vec<Param>) -> Result<Response, Error> {

		self.last_id += 1;
		if self.last_id == 1000 {
			self.last_id = 0;
		}

		let command = Command {
			id: self.last_id,
			method: method.to_owned(),
			params
		};

		let mut payload = serde_json::to_string(&command).unwrap();

		payload.push_str("\r\n");
		let payload = payload;

		//println!("{}", payload);

		self.socket.write(payload.as_bytes())?;

		let mut buf = [0; 128];
		loop {
			//read_to_string here causes it to read null bytes indefinitely
			let len = self.socket.read(&mut buf)?;

			let response = String::from_utf8(buf[0..len].to_vec()).unwrap();

			// if it doesn't include id it's prob a prop update
			if response.contains("id") {
				let result: Response = serde_json::from_str(&response[0..response.len()-2])?;
				if result.id == self.last_id {
					return Ok(result);
				}
			}
		}
	}

	pub fn toggle(&mut self) -> Result<Response, Error> {
		let res = self.send_command("toggle", vec![])?;
		Ok(res)
	}

	pub fn set_bright(&mut self, brightness: u32, effect: &str, duration: u32) -> Result<Response, Error> {
		let res = self.send_command("set_bright", vec![Param::Int(brightness), Param::String(effect.to_owned()), Param::Int(duration)])?;
		Ok(res)
	}
 
}
