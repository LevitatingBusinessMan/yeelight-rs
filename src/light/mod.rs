use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::io::Error;
use std::io;
use regex::Regex;
use std::net::TcpStream;
use std::io::{Write,Read};

pub struct Light {

	pub headers: HashMap<String,String>,
	pub ip: Ipv4Addr,
	pub port: u16,
	pub socket: TcpStream

}

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

		let mut socket = TcpStream::connect((ip, port)).expect("Failed connecting to the server");

		Ok(Light{headers, ip, port, socket})
	}

	pub fn toggle(&mut self) -> Result<(), Error> {
		println!("Attempting toggle");
		self.socket.write(b"{\"id\":0, \"method\":\"toggle\", \"params\":[]}\r\n");

		let mut buf = [0; 128];
		self.socket.read(&mut buf)?;
		println!("{}",std::str::from_utf8(&buf).unwrap());

		Ok(())
	}

}
