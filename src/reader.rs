use std::io::{BufReader};
use std::sync::mpsc::{Sender};
use network::PacketReader;
use ::capnp::serialize_packed;
//use ::capnp::message::Builder;
//use ::connections_capnp::connection_request;
use ::connections_capnp::app_packet;
use connstate::ClientRegistryKeeper;
use connstate::SocketReadAddress;

pub trait CommCommand {
    fn execute(&self, comm_context: & mut ClientRegistryKeeper);
}

pub struct PacketReaderServer {
    command_tx : Sender<Box<CommCommand + Send>>,
//let (tx,rx): (Sender<Box<Command + Send>>, Receiver<Box<Command + Send>>) = mpsc::channel();

}

//pub struct CommCommandProcessor {
//    command_rx : Receiver<Box<CommCommand + Send>>,
//}

struct AddClientCommand {
    read_host : String,
    read_port : String
}

impl CommCommand for AddClientCommand {
    fn execute(&self, clients: & mut ClientRegistryKeeper) {
        eprintln!("!!!!!!!!!!!!!!!! READ HOST: {:?}", self.read_host);
        eprintln!("!!!!!!!!!!!!!!!! READ HOST: {:?}", self.read_port);
        let client_handle : SocketReadAddress =  SocketReadAddress {
            read_host: self.read_host.to_string(),
            _read_port: self.read_port.parse::<u32>().unwrap()};
        clients.add_client(client_handle);
//        unimplemented!()
    }
}

impl PacketReaderServer {
    pub fn with_sender(sender: Sender<Box<CommCommand + Send>>) -> PacketReaderServer {
        PacketReaderServer {command_tx: sender}
    }
}
impl PacketReader for PacketReaderServer {

    // read_command_packet = This lives in the 'reader' thread and does the necessary
    // deserialization.  If the
    fn read_command_packet(&self, buf: &[u8; 2048]) -> ::capnp::Result<()> {
        //let stdin = ::std::io::stdin();

        //    let mut br = BufReader::new(buf.as_ref());
        //let mut br = BufReader::new(buf.as_ref());

        let slice: &[u8] = buf;
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
                let cmd = Box::new(AddClientCommand{read_host: String::from(request.get_client_read_host()?),
                    read_port: String::from(request.get_client_read_port()?)});

                self.command_tx.send(cmd).unwrap();
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
    use network::{write_packet_to_buffer, send_packet_to_socket, read_packets};
    use std::sync::mpsc;
    use std::sync::mpsc::{Sender, Receiver};
    use std::collections::HashMap;
//    use std::fmt::Debug;

    #[derive(Debug)]
    struct NameCommand {
        name: String
    }

    #[derive(Debug)]
    struct AgeCommand {
        age: u32
    }

    trait Command {
        fn doit(&self);
    }

    impl Command for NameCommand {
        fn doit(&self) {
            eprintln!("Do it: {}", self.name);
        }
    }

    impl Command for AgeCommand {
        fn doit(&self) {
            eprintln!("Age do it: {}", self.age);
        }
    }


    #[test]
    fn it_works_for_real() {

        let (tx,command_rx): (Sender<Box<CommCommand + Send>>, Receiver<Box<CommCommand + Send>>) = mpsc::channel();

        let pri = PacketReaderServer {command_tx: tx};

        let listen_address =SocketReadAddress{
            read_host: String::from("127.0.0.1"),
            _read_port: 34256
        };

        let rthread = read_packets(pri, &listen_address);
        let ten_millis  = time::Duration::from_millis(200);

        thread::sleep(ten_millis);
        let mut writebuf : [u8;2048] = [0;2048];
        // Works.
        let _ = write_packet_to_buffer(& mut writebuf);
        let _ = send_packet_to_socket(& mut writebuf);

        // Doesn't work. Why?
//        write_packet();

        let received = command_rx.recv().unwrap();

        let mut client_map = HashMap::new();
        received.execute(& mut client_map);
        //command_rx : Receiver<Box<Command + Send>>


        let _ = rthread.join();
        println!("hubba hubba");
        eprintln!("Ehubba hubba");
    }


    // Simple test to work out rust's message passing. key = Box type
    #[test]
    fn channel_comm() {
        let (tx,rx): (Sender<Box<Command + Send>>, Receiver<Box<Command + Send>>) = mpsc::channel();
        let handle = thread::spawn(move || {
            let name_cmd = Box::new(NameCommand{ name: String::from("booga")});
            let age_cmd = Box::new(AgeCommand{ age: 34});
            tx.send(name_cmd).unwrap();
            tx.send(age_cmd).unwrap();
        });

        let mut received = rx.recv().unwrap();
        //received = rx.recv().unwrap();
        received.doit();
        received = rx.recv().unwrap();
        received.doit();
//        eprintln!("Got: {:?}", received);

        let _ =  handle.join();
    }

}


