extern crate capnp;

use std::net::UdpSocket;
use std::io::{BufReader};

use self::capnp::serialize_packed;


//
// Designed to be used inside a thread, 'readPackets()' will read packets until it doesn't
//
fn read_packets() {
    let socket = UdpSocket::bind("127.0.0.1:34256").expect("couldn't bind to address");
    let mut buf = [0; 1024];
    loop {
        println!("socket waiting");
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf)
                                            .expect("Didn't receive data");
        println!("read done");
        if number_of_bytes < 4 {
            println!("recv_from expected >=4 but amount was {}", number_of_bytes);
            break;
        } else {
            println!("amount was{}", number_of_bytes);
            break;
            //read_command_packet(buf);
            //readCommandPacket(
        }
    }
}


fn write_packet() {
    println!("Writing data");
    let socket = UdpSocket::bind("127.0.0.1:34257").expect("couldn't bind to address");
    socket.connect("127.0.0.1:34256").expect("connect function failed");
    socket.send(&[0, 1, 2]).expect("couldn't send message");
    println!("Data Written");
}
/*
int PacketWriter::writeCommandPacket(
shared_ptr<Command> command,
string& host,
int port) {

flatbuffers::FlatBufferBuilder& fbb = command->getBufferBuilder();
int size = fbb.GetSize() + 4; // Need room for the type
if (size > 1000) {
cerr << " Warning: packet size too large" << endl;
}

uint8_t* pkt_notype = fbb.GetBufferPointer();
uint8_t pkt[1024];
uint32_t commandType = htonl(command->getCommandTypeId());

memcpy(pkt, &commandType, 4);
memcpy(pkt+4, pkt_notype, fbb.GetSize());


int sock_fd;
sock_fd = socket(AF_INET, SOCK_DGRAM,0);
struct sockaddr_in server; //, from;
server.sin_family = AF_INET;
//server.sin_7port = htons(port);
server.sin_port = htons(port);
memset(&(server.sin_zero), 0, 8);
server.sin_addr.s_addr = inet_addr(host.c_str());


if (sock_fd < 0) {
cerr << "socket(..) failed" << endl;
}

int n=sendto(sock_fd, pkt,size, 0, (const struct sockaddr *)&server, sizeof(server));

return n;
}
*/


fn read_command_packet(buf : & [u8;1024]) -> capnp::Result<()>  {
    //let stdin = ::std::io::stdin();

//    let mut br = BufReader::new(buf.as_ref());
    //let mut br = BufReader::new(buf.as_ref());

    let mut slice: &[u8] = buf;
    let mut br = BufReader::new( slice ); //buf.as_ref());

    let message_reader = try!(serialize_packed::read_message(&mut br, //&mut stdin.lock(),
                                                             self::capnp::message::ReaderOptions::new()));
    //let address_book = try!(message_reader.get_root::<address_book::Reader>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    fn it_works_for_real() {
        let rthread = thread::spawn(|| {
            read_packets();
        });
        let ten_millis  = time::Duration::from_millis(200);

        thread::sleep(ten_millis);
        write_packet();

        rthread.join();
        println!("hubba hubba");
    }
}


/*void PacketReader::readCommandPacket(uint32_t command_type,
uint8_t *buf, int bufsize,
shared_ptr<map<uint32_t, function<shared_ptr<Command>()>>> cmdCreators) {

shared_ptr<Command> cmd;
if (cmdCreators->find(command_type) == cmdCreators->end()) {
cerr << "Packet handler not found for cmd type: " << command_type<< endl;
} else {
cmd = (*cmdCreators)[command_type]();

if (cmd) {
cmd->writeReceivedData(buf, bufsize);
} else {
cerr << "Command not create but cmd creation lambda does exist.  cmd type: " <<
command_type << endl;
}

cerr << "Command packet read. Returning cmd." << endl;

// Put the command in the queue
destination_queue->enqueue(cmd);
}
*/
