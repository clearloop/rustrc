pub extern crate futures;
pub extern crate bytes;
pub extern crate tokio;
    
pub mod shared;

// Shorthand for the transmit half of the message channel.
pub type Tx = futures::sync::mpsc::UnboundedSender<bytes::Bytes>;

// Shorthand for the receive half of the message channel.
pub type Rx = futures::sync::mpsc::UnboundedReceiver<bytes::Bytes>;


