pub extern crate tokio;
#[macro_use]
pub extern crate futures;
pub extern crate bytes;

    
pub mod shared;
pub mod lines;
pub mod peer;

// Shorthand for the transmit half of the message channel.
pub type Tx = futures::sync::mpsc::UnboundedSender<bytes::Bytes>;




