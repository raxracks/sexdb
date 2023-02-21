use std::collections::HashMap;
use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::{accept, Message};

static ADDRESS: &str = "127.0.0.1:9001";

fn main() {
    let server = TcpListener::bind(ADDRESS).unwrap();
    println!("Running on ws://{}", ADDRESS);
    for stream in server.incoming() {
        let mut data = HashMap::<String, String>::new();
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();

                if msg.is_binary() || msg.is_text() {
                    let split = msg.to_text().unwrap().split(" ").collect::<Vec<&str>>();
                    let id = split[0].to_string();
                    let instruction = split[1].to_uppercase();

                    websocket
                        .write_message(Message::Text(match instruction.as_str() {
                            "SET" => {
                                let key = split[2].to_string();
                                let value = split[3..].join(" ");

                                data.insert(key, value);
                                format!("{} SUCCESS", id)
                            }
                            "GET" => {
                                let key = split[2].to_string();

                                format!(
                                    "{} {}",
                                    id,
                                    data.get(&key)
                                        .unwrap_or(&"ERROR: Key not in store".to_string())
                                        .to_string()
                                )
                            }
                            _ => format!("{} ERROR: Invalid instruction", id),
                        }))
                        .unwrap();
                }
            }
        });
    }
}
