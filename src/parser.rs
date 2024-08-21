use std::{
    io::{self},
    iter::Peekable,
};

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

impl<I> Iterator for KvReqParser<I>
where
    I: Iterator<Item = io::Result<u8>>,
{
    type Item = io::Result<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        let resp = match self.iter.next()? {
            Ok(byte) => {
                // let a = ascii::escape_default(byte);
                // print!("{} <-> {}\n", byte, a);
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
        };

        resp
    }
}

pub fn t<I>(buf_stream: I)
where
    I: Iterator<Item = io::Result<u8>>,
{
    let result: Vec<u8> = KvReqParser::new(buf_stream)
        .collect::<io::Result<Vec<u8>>>()
        .unwrap();

    println!("result: {:?}", String::from_utf8_lossy(&result));
}

#[test]
fn test_single_req() {
    let single_request_byte = b"\r:get a:\n";
    let expected = b"get a:\n";

    let stream = Cursor::new(single_request_byte).bytes();
    let parsed_stream = KvReqParser::new(stream)
        .collect::<io::Result<Vec<u8>>>()
        .expect("failed to parse given byte stream");
    assert_eq!(parsed_stream, expected);
}

#[test]
fn test_multiple_reqs() {
    let multiple_requests_byte = b"\r:set key val:\r:get key:";
    let expected = b"set key val:get key:";

    let stream = Cursor::new(multiple_requests_byte).bytes();
    let parsed_stream = KvReqParser::new(stream)
        .collect::<io::Result<Vec<u8>>>()
        .expect("failed to parse given byte stream");
    assert_eq!(parsed_stream, expected);

    let multiple_requests_byte = b"\r:set key val:\r:get key:\r:rm key:";
    let expected = b"set key val:get key:rm key:";

    let stream = Cursor::new(multiple_requests_byte).bytes();
    let parsed_stream = KvReqParser::new(stream)
        .collect::<io::Result<Vec<u8>>>()
        .expect("failed to parse given byte stream");
    assert_eq!(parsed_stream, expected);
}
