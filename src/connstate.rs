
use std::collections::HashMap;

// Module for managing connection state
//
// Protocol
// --------
//  Client Addition:   Clients connect up with the server and add themselves. [CONN REQUEST]
//                     Once added a confirmation is sent. [CONN MADE]
//
//  Client Removal: Only the server drops a client.  A client can be dropped because of two
//                  events:   1. Client requests disconnect   OR   2. Server determined client
//                  invalid.
//



#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct SocketReadAddress {
    pub read_host: String,
    pub read_port: u32,
    pub reader_id: String
}


pub trait ClientRegistryKeeper {
    fn add_client(& mut self, handle: SocketReadAddress);
    fn remove_client(& mut self, handle: SocketReadAddress);
}

impl ClientRegistryKeeper for HashMap<String, SocketReadAddress> {
    fn add_client(& mut self, handle: SocketReadAddress) {
        self.insert(String::from(handle.read_host.to_string() + ":" + handle.read_port.to_string().as_str()), handle);
    }

    fn remove_client(& mut self, handle: SocketReadAddress) {
        self.remove(&handle.read_host);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_registry() {

        let mut client_registry = HashMap::new();
        let chandle = SocketReadAddress {  read_host: String::from("testhost"), read_port: 55,
            reader_id: String::from("")};
        client_registry.add_client(chandle);
    }


}