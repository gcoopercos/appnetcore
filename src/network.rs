
use std::net::UdpSocket;

use ::capnp::serialize_packed;
use ::capnp::message::Builder;

use std::thread;
use std::io;
//use ::connections_capnp::connection_request;
use ::connections_capnp::app_packet;


pub trait PacketReader {
    fn read_command_packet(&self, buf : & [u8;2048]) -> ::capnp::Result<()>;
}

//
// Spawns off a thread to process incoming packets from a socket and call a function to
// interpret them
// Note on 'static:
// There is no way around this in safe Rust. If you want to pass data to other threads using spawn,
// it must be 'static, period
//
pub fn read_packets<T: PacketReader + Send + 'static>( packet_reader: T) -> thread::JoinHandle<()> {
    let rthread = thread::spawn( move|| {
        let socket = UdpSocket::bind("127.0.0.1:34256").expect("couldn't bind to address");
        let mut buf:[u8;2048] = [0; 2048];
        loop {
//            println!("socket waiting");
            let (number_of_bytes, _src_addr) = socket.recv_from(&mut buf)
                .expect("Didn't receive data");
//            println!("read done");
            if number_of_bytes < 4 {
                println!("recv_from expected >=4 but amount was {}", number_of_bytes);
                break;
            } else {
//                eprint!("READ BUF: ");
//                for i in buf.iter() {
//                    eprint!("{:?}", i)
//                }
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
//    eprintln!("Writing data");
    let socket = UdpSocket::bind("127.0.0.1:34257").expect("couldn't bind to address");
    socket.connect("127.0.0.1:34256").expect("connect function failed");
    let mut bufslice = & mut buf[..];
//    eprint!("BUF to WRITE: ");
//    for i in bufslice.iter() {
//        eprint!("{:?}", i)
//    }
    socket.send(&mut bufslice).expect("couldn't send packet");
//    eprintln!("Data Written");
}

//fn write_packet() {
//    eprintln!("Writing data");
//    let socket = UdpSocket::bind("127.0.0.1:34257").expect("couldn't bind to address");
//    socket.connect("127.0.0.1:34256").expect("connect function failed");
////    socket.send(&[0, 1, 2]).expect("couldn't send message");
//    eprintln!("Data Written");
//
//    let mut message = Builder::new_default();
//    {
////        let mut cr = message.init_root::<connection_request::Builder>();
//        let app_packet = message.init_root::<app_packet::Builder>();
//        let packetType = app_packet.get_packet_type(); //init_packet_type();
//        let mut cr = packetType.init_connection_request();
//        cr.set_client_read_host("testhost22");
//        cr.set_client_read_port("1234");
//        cr.set_client_name("cname");
//        cr.set_client_pass("cpass");
//    }
//    let mut buf :[u8; 2048] = [0;2048];
//    let mut bufslice = & mut buf[..];
//
//    serialize_packed::write_message( & mut bufslice, & mut message);
////    serialize_packed::write_message( & mut buf, & mut message);
//    eprint!("BUF to SEND (write_packet): ");
//    for i in bufslice.iter() {
//        eprint!("{:?}", i)
//    }
//
//    socket.send(&mut bufslice).expect("couldn't send packet");
////    socket.send(&buf).expect("couldn't send packet");
//
//}

