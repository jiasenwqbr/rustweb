use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};
fn main() {
    let mut _stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    _stream.write("hello world".as_bytes()).unwrap();
    let mut buf = [0; 1024];
    _stream.read(&mut buf).unwrap();
    println!("response from server : {}", str::from_utf8(&buf).unwrap());
}
