
use std::net::UdpSocket;

use ::capnp::serialize_packed;
use ::capnp::message::Builder;

use ::connections_capnp::app_packet;

pub struct PacketWriter {
    client_read_host: String,
    client_read_port: String,
    client_name: String,
    client_pass: String,
    dest_host: String,
    dest_port: String,
    socket: UdpSocket
}

impl PacketWriter {
    pub fn with_destination(
        client_read_host: &str,
        client_read_port: &str,
        client_write_port: &str,
        client_name: &str,
        client_pass: &str,
        dest_host: &str,
        dest_port: &str) -> PacketWriter
    {
        //let socket = UdpSocket::bind(my_host.to_string() + ":" +
        //   &my_port.to_string()).expect("couldn't bind to address");
        //let bindsocket = UdpSocket::bind(client_read_host.to_string() + ":" +
        //   &client_read_port.to_string()).expect("couldn't bind to address");
        let bindsocket = UdpSocket::bind(client_read_host.to_string() +
            ":" + client_write_port).expect("couldn't bind to address");

        bindsocket.connect(dest_host.to_string() + ":" + &dest_port.to_string()).
            expect("connect function failed");

        PacketWriter {
            client_read_host: String::from(client_read_host),
            client_read_port: String::from(client_read_port),
            client_name: String::from(client_name),
            client_pass: String::from(client_pass),
            dest_host: String::from(dest_host),
            dest_port: String::from(dest_port),
            socket: bindsocket
        }
    }

    pub fn send_connection_request(&self)
    {
        let mut writebuf: [u8; 2048] = [0; 2048];
        // Works.

        let mut message = Builder::new_default();
        {
            let app_packet = message.init_root::<app_packet::Builder>();
            let packet_type = app_packet.get_packet_type(); //init_packet_type();
            let mut cr = packet_type.init_connection_request();
            cr.set_client_read_host(&self.client_read_host);
            cr.set_client_read_port(&self.client_read_port);
            cr.set_client_name(&self.client_name);
            cr.set_client_pass(&self.client_pass);
        }
        {
            let mut bufslice = &mut writebuf[..]; //buf[..];

            // TODO better error handling here
            let _ = serialize_packed::write_message(&mut bufslice, &mut message).unwrap();
        }
        self.send_packet_to_socket(&self.dest_host, &self.dest_port, &writebuf);
    }

    pub fn send_text_message(&self, receiver_id: &str, text_message: &str) {
        let mut writebuf: [u8; 2048] = [0; 2048];
        // Works.

        let mut message = Builder::new_default();
        {
            let app_packet = message.init_root::<app_packet::Builder>();
            let packet_type = app_packet.get_packet_type(); //init_packet_type();
            let mut tm = packet_type.init_text_message();
            tm.set_receiver_id(receiver_id);
            tm.set_message(text_message);
        }
        {
            let mut bufslice = &mut writebuf[..]; //buf[..];

            // TODO better error handling here
            let _ = serialize_packed::write_message(&mut bufslice, &mut message).unwrap();
        }
        self.send_packet_to_socket(&self.dest_host, &self.dest_port, &writebuf);

    }

    pub fn send_packet_to_socket(&self, 
                             _dest_host: &str, _dest_port: &str,
                             buf : &   [u8;2048]) {
        let  bufslice = &  buf[..];
        self.socket.send(& bufslice).expect("couldn't send packet");
    }

}




