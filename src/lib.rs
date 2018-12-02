extern crate capnp;
pub mod reader;
pub mod writer;
pub mod network;
pub mod connstate;

//pub mod connection_request_capnp {
pub mod connections_capnp {
    include!(concat!(env!("OUT_DIR"), "/connections_capnp.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
