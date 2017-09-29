extern crate capnp;
mod reader;
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
