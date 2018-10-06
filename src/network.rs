
use std::net::UdpSocket;
use std::io::{BufReader};

use ::capnp::serialize_packed;
use ::capnp::message::Builder;

use std::thread;

//use ::connections_capnp::connection_request;
use ::connections_capnp::app_packet;


pub trait PacketReader {
    fn read_command_packet(&self, buf : & [u8;2048]) -> ::capnp::Result<()>;
}

//
// Spawns off a thread to process incoming packets from a socket and call a function to
// interpret them
// Note on 'static:
// There is no way around this in safe Rust. If you want to pass data to other threads using this function, it must be 'static, period
//
pub fn read_packets<T: PacketReader + Send + 'static>( packet_reader: T) -> thread::JoinHandle<()> {
    let rthread = thread::spawn( move|| {
        let socket = UdpSocket::bind("127.0.0.1:34256").expect("couldn't bind to address");
        let mut buf:[u8;2048] = [0; 2048];
        loop {
            println!("socket waiting");
            let (number_of_bytes, _src_addr) = socket.recv_from(&mut buf)
                .expect("Didn't receive data");
            println!("read done");
            if number_of_bytes < 4 {
                println!("recv_from expected >=4 but amount was {}", number_of_bytes);
                break;
            } else {
                eprint!("READ BUF: ");
                for i in buf.iter() {
                    eprint!("{:?}", i)
                }
                let res = packet_reader.read_command_packet(&buf);
                match res {
                    Ok(_val) =>  println!("amount was {}", number_of_bytes),
                    Err(E) => println!("Problem {:?}  bytes: {}", E, number_of_bytes),
                }
                break;
                //readCommandPacket(
            }
        }
    });

    rthread
}


pub fn write_packet_to_buffer(buf : &  mut [u8;2048]) {
    let mut message = Builder::new_default();
    {
        //        let mut cr = message.init_root::<connection_request::Builder>();
        let mut app_packet = message.init_root::<app_packet::Builder>();
        let mut packetType = app_packet.get_packet_type(); //init_packet_type();
        let mut cr = packetType.init_connection_request();
        cr.set_client_read_host("testhost22");
        cr.set_client_read_port("1234");
        cr.set_client_name("cname");
        cr.set_client_pass("cpass");
    }
//    let mut buf :[u8; 2048] = [0;2048];
    let mut bufslice = & mut buf[..];

    serialize_packed::write_message( & mut bufslice, & mut message);
}

pub fn send_packet_to_socket(buf : &  mut [u8;2048]) {
    eprintln!("Writing data");
    let socket = UdpSocket::bind("127.0.0.1:34257").expect("couldn't bind to address");
    socket.connect("127.0.0.1:34256").expect("connect function failed");
    let mut bufslice = & mut buf[..];
    eprint!("BUF to WRITE: ");
    for i in bufslice.iter() {
        eprint!("{:?}", i)
    }
    socket.send(&mut bufslice).expect("couldn't send packet");
    eprintln!("Data Written");
}

fn write_packet() {
    eprintln!("Writing data");
    let socket = UdpSocket::bind("127.0.0.1:34257").expect("couldn't bind to address");
    socket.connect("127.0.0.1:34256").expect("connect function failed");
//    socket.send(&[0, 1, 2]).expect("couldn't send message");
    eprintln!("Data Written");

    let mut message = Builder::new_default();
    {
//        let mut cr = message.init_root::<connection_request::Builder>();
        let mut app_packet = message.init_root::<app_packet::Builder>();
        let mut packetType = app_packet.get_packet_type(); //init_packet_type();
        let mut cr = packetType.init_connection_request();
        cr.set_client_read_host("testhost22");
        cr.set_client_read_port("1234");
        cr.set_client_name("cname");
        cr.set_client_pass("cpass");
    }
    let mut buf :[u8; 2048] = [0;2048];
    let mut bufslice = & mut buf[..];

    serialize_packed::write_message( & mut bufslice, & mut message);
//    serialize_packed::write_message( & mut buf, & mut message);
    eprint!("BUF to SEND (write_packet): ");
    for i in bufslice.iter() {
        eprint!("{:?}", i)
    }

    socket.send(&mut bufslice).expect("couldn't send packet");
//    socket.send(&buf).expect("couldn't send packet");

}


//fn read_command_packet(buf : & [u8;2048]) -> ::capnp::Result<()>  {
//    //let stdin = ::std::io::stdin();
//
////    let mut br = BufReader::new(buf.as_ref());
//    //let mut br = BufReader::new(buf.as_ref());
//
//    let mut slice: &[u8] = buf;
//    let mut br = BufReader::new( slice ); //buf.as_ref());
//
//    println!("Deserializing message...");
//    let message_reader = try!(serialize_packed::read_message(&mut br,
//                                                         ::capnp::message::ReaderOptions::new()));
////          let address_book = try!(message_reader.get_root::<address_book::Reader>());
//
//    println!("Getting app packet...");
//    let app_packet = try!(message_reader.get_root::<app_packet::Reader>());
//
//    println!("Determining app packet type...");
//    match app_packet.get_packet_type().which() {
//        Ok(app_packet::packet_type::ConnectionRequest(_cr)) => {
//            println!("");println!("  ConnectRequest!! ");
//            let request = _cr?;
//            println!("CR: {}", request.get_client_name()?);
//            println!("CR: {}", request.get_client_pass()?);
//            println!("CR: {}", request.get_client_read_host()?);
//            println!("CR: {}", request.get_client_read_port()?);
//        }
//        Ok(app_packet::packet_type::HelloMessage(_msg)) => {
//            println!("  msg: {}", "Hellow Message!!!");
//        }
//        Err(::capnp::NotInSchema(_)) => { println!(" Not in schema error")}
//    }
////    let conmessage_reader.get_root::<connection_request::Reader>();
//    //let address_book = try!(message_reader.get_root::<address_book::Reader>());
//    Ok(())
//}

//#[cfg(test)]
//mod tests {
//    use super::*;
//    use std::{thread, time};
//
//    #[test]
//    fn it_works_for_real() {
//        let rthread = thread::spawn(|| {
//            read_packets();
//        });
//        let ten_millis  = time::Duration::from_millis(200);
//
//        thread::sleep(ten_millis);
//        let mut writebuf : [u8;2048] = [0;2048];
//        // Works.
//        write_packet_to_buffer(& mut writebuf);
//        send_packet_to_socket(& mut writebuf);
//
//        // Doesn't work. Why?
////        write_packet();
//
//        rthread.join();
//        println!("hubba hubba");
//        eprintln!("Ehubba hubba");
//    }
//
////    #[test]
////    fn serdes_data() {
////        eprintln!("Ser data...");
////        let mut writebuf : [u8;2048] = [0;2048];
////        write_packet_to_buffer(& mut writebuf);
////
////        eprintln!("Des data...");
////        let mut readbuf : [u8;2048] = [0;2048];
////        let res = read_command_packet(&writebuf);
////        match res {
////            Ok(_val) =>  println!("Read ok!"),
////            Err(E) => println!("Problem {:?} ", E),
////        }
////
////        eprintln!("Data Read.");
////        //for (i, elem) in readbuf.iter_mut().enumerate() {
////        for i in writebuf.iter() {
////            eprint!("{:?}", i)
////        }
////    }
//}
//
