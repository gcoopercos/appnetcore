use std::io::{BufReader};
use std::collections::HashMap;

use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use network::PacketReader;
use ::capnp::serialize_packed;
use ::connections_capnp::app_packet;
use connstate::ClientRegistryKeeper;
use connstate::SocketReadAddress;

/// Communication related commands. Connection, disconnection, etc...
pub trait CommCommand {
    fn execute(&self, comm_context: & mut ClientRegistryKeeper);
}

/// Application related commands. App
pub trait AppCommand {
    fn execute(&self  /*, TODO app_conext here */);
}


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
    }
}

struct TextMessageCommand {
    receiver_id: u32,
    message : String
}

impl AppCommand for TextMessageCommand {
    fn execute(&self) {
        eprint!("{}", self.message);
        //eprintln!("Message: {}", self.message);
//        eprintln!("Receiver_id: {}", self.receiver_id);
    }
}

pub struct PacketReaderServer {
    command_tx : Sender<Box<CommCommand + Send>>,
    app_tx : Sender<Box<AppCommand + Send>>
}

impl PacketReaderServer {
    pub fn with_senders(
        comm_sender: Sender<Box<CommCommand + Send>>,
        app_sender: Sender<Box<AppCommand + Send>>) -> PacketReaderServer
    {
        PacketReaderServer {app_tx: app_sender, command_tx: comm_sender}
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

        //eprintln!("Deserializing message...");
        let message_reader = try!(serialize_packed::read_message(&mut br,
                                                                 ::capnp::message::ReaderOptions::new()));
        //          let address_book = try!(message_reader.get_root::<address_book::Reader>());

        //eprintln!("Getting app packet...");
        let app_packet = try!(message_reader.get_root::<app_packet::Reader>());

        // eprintln!("Determining app packet type...");
        match app_packet.get_packet_type().which() {
            Ok(app_packet::packet_type::ConnectionRequest(_cr)) => {
                println!("  ConnectRequest!! ");
                let request = _cr?;
                println!("CR: {}", request.get_client_name()?);
                println!("CR: {}", request.get_client_pass()?);
                println!("CR: {}", request.get_client_read_host()?);
                println!("CR: {}", request.get_client_read_port()?);
                let cmd =
                    Box::new(AddClientCommand {
                        read_host: String::from(request.get_client_read_host()?),
                        read_port: String::from(request.get_client_read_port()?)
                    });

                self.command_tx.send(cmd).unwrap();
            }
            Ok(app_packet::packet_type::ConnectionResponse(_msg)) => {
                println!("  msg: {}", "Connection Response Received");
            }
            Ok(app_packet::packet_type::TextMessage(msg)) => {
                let text_message = msg?;
                let cmd =
                    Box::new(TextMessageCommand {
                        receiver_id: text_message.get_receiver_id(),
                        message: String::from(text_message.get_message()?)
                    });

                self.app_tx.send(cmd).unwrap();
            }
            Err(::capnp::NotInSchema(_)) => { println!(" Not in schema error") }
        }
        //    let conmessage_reader.get_root::<connection_request::Reader>();
        //let address_book = try!(message_reader.get_root::<address_book::Reader>());
        Ok(())
    }
}

//
// Grabs 1 command off the comm channel and executes it.
//
pub fn check_comm_commands(rx: &Receiver<Box<CommCommand + Send>>,
                       client_state: & mut HashMap<String,SocketReadAddress>) -> Result<Box<CommCommand>, TryRecvError> {
    let received_value = rx.try_recv()?;
    received_value.execute(client_state);
    Ok(received_value)
}

//
// Processes application level commands
//
pub fn check_app_commands(rx: &Receiver<Box<AppCommand + Send>>,)
                          -> Result<Box<AppCommand>, TryRecvError> {
    let received_value = rx.try_recv()?;
    received_value.execute();
    Ok(received_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};
    use network::{write_packet_to_buffer, send_packet_to_socket, read_packets};
    use writer::PacketWriter;
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
        let (app_tx,_app_command_rx): (Sender<Box<AppCommand + Send>>, Receiver<Box<AppCommand + Send>>) = mpsc::channel();

        let pri = PacketReaderServer {app_tx: app_tx, command_tx: tx};

        let listen_address =SocketReadAddress{
            read_host: String::from("127.0.0.1"),
            _read_port: 34256
        };

        let rthread = read_packets(pri, &listen_address);
        let ten_millis  = time::Duration::from_millis(200);

        thread::sleep(ten_millis);
        let mut writebuf : [u8;2048] = [0;2048];
        // Works.

        let packet_writer = PacketWriter::with_destination(
            "127.0.0.1",
            "34527",
            "cname",
            "pass",
            "127.0.0.1",
            "34256");

        packet_writer.send_connection_request();
        /*
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
    let mut bufslice = & mut buf[..];*/








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


