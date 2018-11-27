
use std::net::UdpSocket;

use ::capnp::serialize_packed;
use ::capnp::message::Builder;

use std::thread;
use std::io;
use ::connections_capnp::app_packet;

use ::connstate::SocketReadAddress;

pub trait PacketReader {
    fn read_command_packet(&self, buf : & [u8;2048]) -> ::capnp::Result<()>;
}

///
/// Spawns off a thread to process incoming packets from a socket and call a function to
/// interpret them
/// Note on 'static:
/// There is no way around this in safe Rust. If you want to pass data to other threads using spawn,
/// it must be 'static, period
///
pub fn read_packets<T: PacketReader + Send + 'static>( packet_reader: T,
                                                       listen_port : &SocketReadAddress) -> thread::JoinHandle<()> {
    let mut addr_str : String = listen_port.read_host.to_string();
    addr_str.push_str(":");
    addr_str.push_str(&listen_port._read_port.to_string());

    let rthread = thread::spawn( move|| {
//        let mut addr_str : String = listen_port.read_host.to_string(); //;;"127.0.0.1:".to_string();
//        addr_str.push_str(":");
//        addr_str.push_str(&listen_port._read_port.to_string());
        let socket = UdpSocket::bind(addr_str).expect("couldn't bind to address");
        let mut buf:[u8;2048] = [0; 2048];
        loop {
            let (number_of_bytes, _src_addr) = socket.recv_from(&mut buf)
                .expect("Didn't receive data");
            if number_of_bytes < 4 {
                println!("recv_from expected >=4 but amount was {}", number_of_bytes);
                break;
            } else {
                let res = packet_reader.read_command_packet(&buf);
                match res {
                    Ok(_val) =>  println!("amount was {}", number_of_bytes),
                    Err(e) => println!("Problem {:?}  bytes: {}", e, number_of_bytes),
                }
                break;
            }
        }
    });

    rthread
}


pub fn write_packet_to_buffer(buf : &  mut [u8;2048]) -> io::Result<()> {
    let mut message = Builder::new_default();
    {
        let app_packet = message.init_root::<app_packet::Builder>();
        let packet_type = app_packet.get_packet_type(); //init_packet_type();
        let mut cr = packet_type.init_connection_request();
        cr.set_client_read_host("testhost22");
        cr.set_client_read_port("1234");
        cr.set_client_name("cname");
        cr.set_client_pass("cpass");
    }
    let mut bufslice = & mut buf[..];

    serialize_packed::write_message( & mut bufslice, & mut message)
}

pub fn send_packet_to_socket(buf : &  mut [u8;2048]) {
    let socket = UdpSocket::bind("127.0.0.1:34257").expect("couldn't bind to address");
    socket.connect("127.0.0.1:34256").expect("connect function failed");
    let mut bufslice = & mut buf[..];
    socket.send(&mut bufslice).expect("couldn't send packet");
}
