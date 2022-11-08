use std::io::{Error, ErrorKind, Read, Write};
use std::net::TcpStream;

fn main() {
    match connect() {
        Ok(_) => std::process::exit(0),
        _ => std::process::exit(1),
    };
}

fn connect() -> Result<(), std::io::Error> {
    let url = std::env::args().nth(1).ok_or(Error::from(ErrorKind::Other))?;
    let endpoint = std::env::args().nth(2).ok_or(Error::from(ErrorKind::Other))?;

    let mut stream = TcpStream::connect(&url)?;

    let mut data = String::new();
    data.push_str("GET ");
    data.push_str(endpoint.as_str());
    data.push_str(" HTTP/1.1\r\nHost: ");
    data.push_str(url.as_str());
    data.push_str("\r\nConnection: close\r\n\r\n");

    stream.write_all(data.as_bytes())?;

    let mut buf = String::new();
    match stream.read_to_string(&mut buf) {
        Ok(_) if buf.contains("200 OK") => Ok(()),
        _ => Err(Error::from(ErrorKind::Other)),
    }
}
