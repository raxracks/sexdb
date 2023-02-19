use std::collections::HashMap;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

/// A WebSocket echo server
fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        let mut data = HashMap::<String, String>::new();
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    let split = msg.to_text().unwrap().split(" ").collect::<Vec<&str>>();
                    let instruction = split[0];
                    let key = split[1].to_string();
                    let value = split[2..].join(" ");
                    let mut result = String::new();

                    match instruction {
                        "SET" => {
                            data.insert(key, value);
                            result = "SUCCESS".to_string();
                            ()
                        }
                        "GET" => {
                            result = data
                                .get(&key)
                                .unwrap_or(&"ERROR: Key not in store".to_string())
                                .to_string();
                            ()
                        }
                        _ => {
                            result = "ERROR: Invalid instruction".to_string();
                            ()
                        }
                    }

                    websocket.write_message(Message::Text(result)).unwrap();
                }
            }
        });
    }
}
