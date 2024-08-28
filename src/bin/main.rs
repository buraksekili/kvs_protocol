use kvs_protocol::{deserializer::deserialize, error::Result, parser, request::Request};
use std::{
    io::{BufReader, Read},
    net::{TcpListener, TcpStream},
};

fn handle_client(stream: &mut TcpStream) {
    println!("******* handling client request **********");
    let mut buf_stream = BufReader::new(stream);
    let mut buf: Vec<u8> = Vec::new();

    match buf_stream.read_to_end(&mut buf) {
        Err(e) => eprintln!("failed to read from buffer, err: {}", e),
        Ok(_) => {}
    }

    let mut parser = parser::KvReqParser::new(&buf);
    while let Some(v) = parser.next() {
        let parsed_str = String::from_utf8_lossy(v);
        println!("parsed stream result => {}", parsed_str);
        match deserialize::<Request>(&parsed_str) {
            Err(e) => eprintln!("failed to deserialize parsed request, err: {}", e),
            Ok(v) => println!("parsed and deserialized request {:?}", v),
        }
    };

    println!("******* DONE **********");
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
