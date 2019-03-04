use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:9333").expect("Could not counnect to server");

    loop {
        let mut input = String::new();
        let mut buffer: Vec<u8> = Vec::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");

        println!("hello input: {:?}", input);

        stream
            .write(input.as_bytes())
            .expect("Failed to write to server");

        let mut reader = BufReader::new(&stream);

        reader
            .read_until(b'\n', &mut buffer)
            .expect("Could not read into buffer");

        println!(
            "{}",
            str::from_utf8(&buffer).expect("Could not write buffer as string")
        );
    }
}
