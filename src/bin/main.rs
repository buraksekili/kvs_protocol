use std::{
    io::{self, BufReader, Read},
    iter::Peekable,
    net::{TcpListener, TcpStream},
};

/// Implements the ability to drop `\r\n` byte pairs from a stream, converting each instance to a single `\n`.
pub struct KvReqParser<I>
where
    I: Iterator<Item = io::Result<u8>>,
{
    iter: Peekable<I>,
}

impl<I> KvReqParser<I>
where
    I: Iterator<Item = io::Result<u8>>,
{
    fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
}

const CR_CHAR: u8 = b'\r';
const COLUMN_CHAR: u8 = b':';

// implement iterator for our struct
impl<I> Iterator for KvReqParser<I>
where
    I: Iterator<Item = io::Result<u8>>,
{
    type Item = io::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next()? {
            Ok(byte) => {
                if byte == CR_CHAR {
                    if let Ok(next) = self.iter.peek()? {
                        if next == &COLUMN_CHAR {
                            self.next();
                            return self.next();
                        }
                    }
                } else if byte == COLUMN_CHAR {
                    match self.iter.peek() {
                        None => self.next(),
                        Some(_) => Some(Ok(byte)),
                    };
                }

                Some(Ok(byte))
            }
            Err(e) => Some(Err(e)),
        }
    }
}

fn handle_client(stream: &mut TcpStream) {
    println!("handle client");
    let buf_stream = BufReader::new(stream).bytes();

    let result = KvReqParser::new(buf_stream)
        .collect::<io::Result<Vec<u8>>>()
        .unwrap();

    println!("result: {:?}", String::from_utf8_lossy(&result));

    println!("**********************");
}

use kvs_protocol::error::Result;

fn main() -> Result<()> {
    // let listener = TcpListener::bind("127.0.0.1:8080")?;

    // // accept connections and process them serially
    // for stream in listener.incoming() {
    //     handle_client(stream?.borrow_mut());
    // }
    return Ok(());
}
