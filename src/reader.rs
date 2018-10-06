
//use std::net::UdpSocket;
use std::io::{BufReader};
use network::PacketReader;
use network::{write_packet_to_buffer, send_packet_to_socket, read_packets};

use ::capnp::serialize_packed;
//use ::capnp::message::Builder;


//use ::connections_capnp::connection_request;
use ::connections_capnp::app_packet;

struct PacketReaderImpl {

}

impl PacketReader for PacketReaderImpl {
    fn read_command_packet(&self, buf: &[u8; 2048]) -> ::capnp::Result<()> {
        //let stdin = ::std::io::stdin();

        //    let mut br = BufReader::new(buf.as_ref());
        //let mut br = BufReader::new(buf.as_ref());

        let mut slice: &[u8] = buf;
        let mut br = BufReader::new(slice); //buf.as_ref());

        println!("Deserializing message...");
        let message_reader = try!(serialize_packed::read_message(&mut br,
                                                                 ::capnp::message::ReaderOptions::new()));
        //          let address_book = try!(message_reader.get_root::<address_book::Reader>());

        println!("Getting app packet...");
        let app_packet = try!(message_reader.get_root::<app_packet::Reader>());

        println!("Determining app packet type...");
        match app_packet.get_packet_type().which() {
            Ok(app_packet::packet_type::ConnectionRequest(_cr)) => {
                println!("");
                println!("  ConnectRequest!! ");
                let request = _cr?;
                println!("CR: {}", request.get_client_name()?);
                println!("CR: {}", request.get_client_pass()?);
                println!("CR: {}", request.get_client_read_host()?);
                println!("CR: {}", request.get_client_read_port()?);
            }
            Ok(app_packet::packet_type::HelloMessage(_msg)) => {
                println!("  msg: {}", "Hellow Message!!!");
            }
            Err(::capnp::NotInSchema(_)) => { println!(" Not in schema error") }
        }
        //    let conmessage_reader.get_root::<connection_request::Reader>();
        //let address_book = try!(message_reader.get_root::<address_book::Reader>());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    fn it_works_for_real() {

        let pri = PacketReaderImpl {};

        let rthread = read_packets(pri);

//
//        let rthread = thread::spawn(|| {
//            read_packets();
//        });
        let ten_millis  = time::Duration::from_millis(200);

        thread::sleep(ten_millis);
        let mut writebuf : [u8;2048] = [0;2048];
        // Works.
        write_packet_to_buffer(& mut writebuf);
        send_packet_to_socket(& mut writebuf);

        // Doesn't work. Why?
//        write_packet();

        rthread.join();
        println!("hubba hubba");
        eprintln!("Ehubba hubba");
    }


}

