use std::collections::HashMap;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        let mut data = HashMap::<String, String>::new();
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                if msg.is_binary() || msg.is_text() {
                    let split = msg.to_text().unwrap().split(" ").collect::<Vec<&str>>();
                    let instruction = split[0];
                    let result: String;

                    match instruction {
                        "SET" => {
                            let key = split[1].to_string();
                            let value = split[2..].join(" ");
                            
                            data.insert(key, value);
                            result = "SUCCESS".to_string();
                            ()
                        }
                        "GET" => {
                            let key = split[1].to_string();
                            
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
