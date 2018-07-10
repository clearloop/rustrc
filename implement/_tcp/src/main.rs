use std::net::TcpListener;

fn main () {
    let _listener = TcpListener::bind("127.0.0.1:89").unwrap();
    println!("\nTCP Server running at 127.0.0.1:89\n");
    for stream in _listener.incoming() {
        println!("{:?}", stream.is_ok());
    }
}
