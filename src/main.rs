mod lib;

// use crate::velocity::database::DatabaseOps;
use wasmedge_sdk::Vm;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use std::vec;
// use velocity::query;

static IP: &str = "0.0.0.0:6379";


fn main() {
    // let mut db = DatabaseOps;
    let listener = TcpListener::bind(IP);
    let listener = match listener {
        Ok(listener) => listener,
        Err(e) => {
            println!("Failed to bind to {}: {}", IP, e.kind());
            return;
        }
    };

    // db.delete_expired_keys();

    println!("Server listening on {}", IP);

    let vm = lib::init_redis_core();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_commands(&stream, &vm);
            }

            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    println!("Server shutting down");
}

fn handle_commands(mut stream: &TcpStream, vm: &Vm) {
    let mut buffer = vec![0; 1024 * 100]; // 100kb buffer

    loop {
        match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                let data = &buffer[..size];
                let request = String::from_utf8_lossy(data);
                println!("{}", request);
                let response = lib::query_and_response(vm, &request);
                println!("{}", response);
                // let query = query::Query::new(&response);
                // let response = query.create_response();

                stream
                    .write_all(&response.as_bytes())
                    .expect("Failed to write to stream");
            }

            Ok(_) => {
                println!("Client closed the connection");
                break;
            }

            Err(e) => {
                println!("Error reading from stream: {}", e);
                break;
            }
        }
    }
}
