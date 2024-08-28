use kvs_protocol::{deserializer::deserialize, error::Result, parser, request::Request};
use std::{
    io::{self, BufReader, Read},
    net::{TcpListener, TcpStream},
};

fn handle_client(stream: &mut TcpStream) {
    let buf_stream: io::Bytes<BufReader<&mut TcpStream>> = BufReader::new(stream).bytes();
    match parser::from_iterator(buf_stream).collect::<io::Result<Vec<u8>>>() {
        Err(e) => eprintln!("failed to parse incoming request, err: {:?}", e),
        Ok(parsed) => {
            let parsed_str = String::from_utf8_lossy(&parsed);
            println!("parsed stream result => {}", parsed_str);

            match deserialize::<Request>(&parsed_str) {
                Err(e) => eprintln!("failed to deserialize parsed request, err: {}", e),
                Ok(v) => println!("parsed and deserialized request {:?}", v),
            }
        }
    };
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("listening on 8080");

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(&mut stream?);
    }

    return Ok(());
}
