use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    println!("Hello, world!");

    // open tcp connection to aprs.glidernet.org:14580
    // send login: user cfi2017 pass -1 vers ogn-rs 0.1 filter r/47.5/7.95/10
    // receive data
    // print data

    let mut stream = TcpStream::connect("aprs.glidernet.org:14580").unwrap();
    stream.write_all(b"user cfi2017 pass -1 vers ogn-rs 0.1 filter r/47.5/7.95/10\n").unwrap();

    // process each line as it comes in
    // if a line is bigger than buffer, continue accumulating

    let mut buffer = [0; 1024];
    let mut accumulator = String::new();
    loop {
        let bytes_read = stream.read(&mut buffer).unwrap();
        for byte in buffer.iter().take(bytes_read) {
            if *byte == b'\n' {
                process(&accumulator);
                accumulator.clear();
            } else {
                accumulator.push(*byte as char);
            }
        }
    }

}

fn process(line: &str) {
    // ignore system messages
    if line.starts_with('#') {
        tracing::trace!("system message: {}", line);
        return;
    }

    tracing::info!("message: {}", line);
}
