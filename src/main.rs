use std::net::{UdpSocket,Ipv4Addr};
use std::io;
use std::collections::HashMap;
use std::time::Duration;

mod light;
use light::Light;

// echo -n "hello" | nc -b -u 127.0.0.1 1982
fn main() -> Result<(), io::Error> {
    let socket = UdpSocket::bind("0.0.0.0:1982")?;

    //Subscribing to the multicast
    let multicast_address = Ipv4Addr::new(239,255,255,250);
    let interface = Ipv4Addr::new(0,0,0,0);
    socket.join_multicast_v4(&multicast_address,&interface)?;
   
    //We don't want to receive our own messages do we?
    socket.set_multicast_loop_v4(false)?;

    discover(&socket);

    let mut buf = [0; 1000];

    let mut lights = vec!();

    socket.set_read_timeout(Some(Duration::new(1, 0)))?;

    loop {
        if let Ok((bytes_received, src_addr)) = socket.recv_from(&mut buf) {
            let buf = &mut buf[..bytes_received]; //Shrink to size
            println!("{:?} {:?}",bytes_received,src_addr);
            //println!("{}", std::str::from_utf8(&buf).unwrap());
            let headers = parser(&buf).expect("Failed parsing");

            if !lights.iter().any(|light: &Light| light.headers.get("Location") == headers.get("Location")) {
                let new_light = Light::new(headers)?;
                lights.push(new_light);
            }

        } else {
            //timeout
            break
        }
    }

    for mut light in lights {
        //light.toggle();
        light.set_bright(10,"smooth", 500);
    }

    Ok(())
}

/**
 * Broadcast a discovery probe to discover devices
 */
fn discover(socket: &UdpSocket) -> Result<(), io::Error>  {
    let discovery_probe = "M-SEARCH * HTTP/1.1\r\n\
    HOST: 239.255.255.250:1982\r\n\
    MAN: \"ssdp:discover\"\r\n\
    ST: wifi_bulb\r\n";

    socket.send_to(discovery_probe.as_bytes(), "239.255.255.250:1982")?;
    Ok(())
}

#[derive(Debug)]
struct ParserError {
    msg: &'static str
}

fn parser(data: &[u8]) -> Result<HashMap<String,String>, ParserError>  {
    let data = std::str::from_utf8(data);
    let data = match data {
        Ok(data) => data,
        Err(error) => return Err(ParserError{msg: "Invalid utf8 character in beacon data"}),
    };

    let mut lines = data.lines();

    if lines.next() != Some("HTTP/1.1 200 OK") {
        return Err(ParserError{msg: "Unexpected response header (NON-HTTP?)"})
    }

    let mut headers = HashMap::new();

    for line in lines {

        let mut l_iter = line.splitn(2, ": ");

        let key = l_iter.next().ok_or(ParserError{msg: "Unable to parse header"})?.to_owned();
        let value = l_iter.next().ok_or(ParserError{msg: "Unable to parse header"})?.to_owned();

        headers.insert(key, value);

    }

    Ok(headers)

}
