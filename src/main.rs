use std::{
    io::{Read, Write},
    net::TcpStream,
};

fn main() -> Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;
    let cmd = "*1\r\n$4\r\nPING\r\n";
    match stream.write(cmd.as_bytes()) {
        Err(err) => println!("failed to send {}, err {}", cmd, err),
        Ok(_) => println!("Send ✅"),
    };
    stream.flush()?;

    let mut buffer = [0; 10];

    match stream.read(&mut buffer) {
        Err(err) => println!("failed to read, err {}", err),
        Ok(read) => println!("Received {:?}", String::from_utf8_lossy(&buffer[..read])),
    }

    Ok(())
}

#[derive(Debug)]
enum RedisClientErr {
    IOError(String),
}

impl From<std::io::Error> for RedisClientErr {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value.to_string())
    }
}
type Result<T> = std::result::Result<T, RedisClientErr>;
