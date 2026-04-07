use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
};

mod handler;
mod leibniz;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3030").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let request_line = handler::read_request_line(&stream);
    let (method, path) = handler::parse_request_line(&request_line);
    let response = handler::route_request(&method, &path);
    stream.write_all(response.as_bytes()).unwrap();
}

