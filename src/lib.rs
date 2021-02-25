use std::net::{UdpSocket, Ipv4Addr};

pub mod parser;
pub mod light;

const DISCOVERY_PROBE: &'static str = "M-SEARCH * HTTP/1.1\r\n\
HOST: 239.255.255.250:1982\r\n\
MAN: \"ssdp:discover\"\r\n\
ST: wifi_bulb\r\n";

/**
 * Broadcast a discovery probe to discover devices
 */
pub fn discover(socket: &UdpSocket) -> Result<(), std::io::Error>  {
    socket.send_to(DISCOVERY_PROBE.as_bytes(), "239.255.255.250:1982")?;
	Ok(())
}

///Helper function that will bind to a UDP broadcasting sockett for you
pub fn bind_broadcast_socket() -> std::io::Result<UdpSocket> {
	let socket = UdpSocket::bind("0.0.0.0:1982")?;

    //Subscribing to the multicast
    let multicast_address = Ipv4Addr::new(239,255,255,250);
    let interface = Ipv4Addr::new(0,0,0,0);
    socket.join_multicast_v4(&multicast_address,&interface)?;
   
    //We don't want to receive our own messages do we?
    socket.set_multicast_loop_v4(false)?;
	Ok(socket)
}
