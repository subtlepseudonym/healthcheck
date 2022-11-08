use std::io::Read;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    let url: String = match std::env::args().nth(1) {
        Some(u) => u,
        _ => std::process::exit(2),
    };

    let endpoint: String = match std::env::args().nth(2) {
        Some(e) => e,
        _ => std::process::exit(2),
    };

    let mut stream = match TcpStream::connect(&url) {
        Ok(s) => s,
        Err(_) => std::process::exit(3),
    };

    let mut data = String::new();
    data.push_str("GET ");
    data.push_str(endpoint.as_str());
    data.push_str(" HTTP/1.1\r\nHost: ");
    data.push_str(url.as_str());
    data.push_str("\r\nConnection: close\r\n\r\n");

    match stream.write_all(data.as_bytes()) {
        Ok(_) => (),
        Err(_) => std::process::exit(4),
    };

    let mut buf = String::new();
    match stream.read_to_string(&mut buf) {
        Ok(_) if buf.contains("200 OK") => std::process::exit(0),
        _ => std::process::exit(5),
    }
}
