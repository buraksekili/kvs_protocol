use std::io::{self};

const STARTING_CMD: &[u8] = b"+:";
const STARTING_CMD_LEN: usize = STARTING_CMD.len();
const COLUMN_CHAR: u8 = b':';

pub struct KvReqParser<'b> {
    bytes: &'b [u8],
    pos: usize,
}

impl<'b> KvReqParser<'b> {
    pub fn new(bytes: &'b [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn find_next_command(&mut self) -> Option<&'b [u8]> {
        loop {
            if self.pos >= self.bytes.len() {
                return None;
            }

            // Find the start marker
            if self.bytes[self.pos..].starts_with(STARTING_CMD) {
                self.pos += STARTING_CMD_LEN; // Skip "+:"
                let start = self.pos;

                // Find the end marker
                match self.bytes[start..].iter().position(|&b| b == COLUMN_CHAR) {
                    Some(end_pos) => {
                        self.pos = start + end_pos + 1; // Move past ":"
                        return Some(&self.bytes[start..start + end_pos]);
                    }
                    None => return None, // No ":" found, malformed input
                }
            }

            self.pos += 1;
        }
    }
}

impl<'b> Iterator for KvReqParser<'b> {
    type Item = &'b [u8];

    fn next(&mut self) -> Option<Self::Item> {
        self.find_next_command()
    }
}

mod tests {
    #[allow(unused_imports)]
    use io::{Cursor, Read};

    use super::*;

    #[test]
    fn test_single_req() {
        let single_request_byte = b"+:get a:";
        let expected = b"get a";

        let parsed_stream = KvReqParser::new(single_request_byte)
            .next()
            .expect("failed to parse given byte stream");
        assert_eq!(parsed_stream, expected);
    }

    #[test]
    fn test_multiple_reqs() {
        let multiple_requests_byte = b"+:set key val:+:get key:+:rm something:";
        let mut parser = KvReqParser::new(multiple_requests_byte);

        let first_cmd = parser.next().expect("failed to parse given byte stream");
        let expected = b"set key val";
        assert_eq!(first_cmd, expected);

        let second_cmd = parser.next().expect("failed to parse given byte stream");
        let expected = b"get key";
        assert_eq!(second_cmd, expected);

        let third_cmd = parser.next().expect("failed to parse given byte stream");
        let expected = b"rm something";
        assert_eq!(third_cmd, expected);
    }
}
