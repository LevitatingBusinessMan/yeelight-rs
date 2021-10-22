use yeelight_rs::{bind_broadcast_socket, discover};
use yeelight_rs::light::Light;
use yeelight_rs::parser::parser;
#[test]
fn main() {

	//Create a socket for discovery
	let socket = bind_broadcast_socket().unwrap();
	discover(&socket).unwrap();

    //This won't go to vec for whatever reason
	let mut buf = [0; 1000];
	let mut lights = Vec::<Light>::new();

	socket.set_read_timeout(Some(std::time::Duration::new(5, 0))).unwrap();

	loop {
        if let Ok((bytes_received, src_addr)) = socket.recv_from(&mut buf) {
            let buf = &mut buf[..bytes_received]; //Shrink to size
			
			println!("{:?} {:?}",bytes_received,src_addr);
            let headers = parser(&buf).expect("Failed parsing");

            if !lights.iter().any(|light: &Light| light.headers.get("Location") == headers.get("Location")) {
                let new_light = Light::new(headers).unwrap();
                lights.push(new_light);
            }

        } else {
            //timeout
            break
        }
    }

    for mut light in lights {
        //light.toggle();
        light.set_bright(10,"smooth", 500).unwrap();
    }

}
