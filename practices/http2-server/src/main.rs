use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream},
};
use http2_server::ThreadPool;

mod handler;
mod leibniz;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); // Four threads 

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

pub fn handle_connection(mut stream: TcpStream) {
    let request_line = handler::read_request_line(&stream);
    let (method, path) = handler::parse_request_line(&request_line);
    let response = handler::route_request(&method, &path);
    stream.write_all(response.as_bytes()).unwrap();
}