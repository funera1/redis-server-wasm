mod lib;

// use chan::chan_select;
// use chan_signal::Signal;
use wasmedge_sdk::{Vm, NeverType};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::vec;
use clap::Parser;

static IP: &str = "0.0.0.0:6379";

fn main() {
    env_logger::init();

    let options = lib::Args::parse();
    log::debug!("image_dir: {}", options.image_dir);
    log::debug!("restore_flag: {}", options.restore_flag);

    // signal handler
    // let s = chan_signal::notify(&[Signal::TERM, Signal::USR1]);
    // chan_select! {
    //     s.recv() -> signal => {
    //         println!("signal={:?}", signal);
    //     }
    // }

    // init WasmVM
    let vm = lib::init_redis_core(&options);

    // setup connection
    let listener = TcpListener::bind(IP);
    let listener = match listener {
        Ok(listener) => listener,
        Err(e) => {
            println!("Failed to bind to {}: {}", IP, e.kind());
            return;
        }
    };


    println!("Server listening on {}", IP);

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

fn handle_commands(mut stream: &TcpStream, vm: &Vm<NeverType>) {
    let mut buffer = vec![0; 1024 * 100]; // 100kb buffer

    loop {
        match stream.read(&mut buffer) {
            Ok(size) if size > 0 => {
                println!("size: {}", size);
                let data = &buffer[..size];
                let request = String::from_utf8_lossy(data);
                println!("request: {}", request);
                let response = lib::query_and_response(vm, &request);
                println!("response: {}", response);

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
