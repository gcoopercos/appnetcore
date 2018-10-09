
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



pub struct ClientHandle {
    client_read_host: String,
    _client_read_port: u32,
}



pub trait ClientRegistryKeeper {
    fn add_client(& mut self, handle: ClientHandle);
    fn remove_client(& mut self, handle: ClientHandle);
}

impl ClientRegistryKeeper for HashMap<String,ClientHandle> {
    fn add_client(& mut self, handle: ClientHandle) {
        self.insert(String::from(handle.client_read_host.as_str()), handle);
    }

    fn remove_client(& mut self, handle: ClientHandle) {
        self.remove(&handle.client_read_host);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_registry() {

        let mut client_registry = HashMap::new();
        let chandle = ClientHandle{  client_read_host: String::from("testhost"), _client_read_port: 55};
        client_registry.add_client(chandle);
    }


}