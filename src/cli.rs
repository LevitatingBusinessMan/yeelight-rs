use std::any;

use yeelight_rs::{bind_broadcast_socket, discover};
use yeelight_rs::light::{Light, Param};
use yeelight_rs::parser::parser;
use std::io::Write as IoWrite;

fn main() {

    let mut args = std::env::args().skip(1);

    if let Some(method) = args.next() {

        let mut params: Vec<Param> = vec![];
        args.for_each(|arg| {
            if arg.chars().all(char::is_numeric) {
                params.push(Param::Int(arg.parse().unwrap()));
            } else {
                params.push(Param::String(arg));
            }
        });

        //Create a socket for discovery
        let socket = bind_broadcast_socket().unwrap();
        discover(&socket).unwrap();

        //This won't go to vec for whatever reason
        let mut buf = [0; 1000];
        let mut lights = Vec::<Light>::new();

        socket.set_read_timeout(Some(std::time::Duration::new(3, 0))).unwrap();

        loop {
            if let Ok((bytes_received, _src_addr)) = socket.recv_from(&mut buf) {
                let buf = &mut buf[..bytes_received]; //Shrink to size
                
                let headers = parser(&buf).expect("Failed parsing");

                if !lights.iter().any(|light: &Light| light.headers.get("Location") == headers.get("Location")) {
                    let mut new_light = Light::new(headers).unwrap();
                    let res = new_light.send_command(&method, params.clone()).expect("Error sending command");
                    
                    //https://github.com/rust-lang/rust/issues/46016
                    let write_res = writeln!(std::io::stdout(),"{}",serde_json::to_string(&res).expect("Unable to parse response"));

                    if write_res.is_err() {
                        std::process::exit(141);
                    } 

                    lights.push(new_light);
                }

            } else {
                //timeout
                break
            }
        }
    } else {
        panic!("No method specified")
    }
}
