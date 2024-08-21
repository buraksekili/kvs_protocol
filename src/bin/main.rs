use std::{
    io::{self, BufReader, Read},
    net::{TcpListener, TcpStream},
};

fn handle_client(stream: &mut TcpStream) {
    println!("handle client");
    let buf_stream: io::Bytes<BufReader<&mut TcpStream>> = BufReader::new(stream).bytes();
    parser::t(buf_stream);

    println!("**********************");
}

use kvs_protocol::{error::Result, parser};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("listening on 8080");

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }

    return Ok(());
}
