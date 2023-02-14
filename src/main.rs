#![allow(dead_code)]

mod config;
mod parse_json;
mod item_processor;

use tungstenite::{connect, Message};
use url::Url;
use crate::item_processor::process_new_listing;
use std::time::Instant;


fn main() {
    println!("SharkPort");
    let config = config::read_config();

    let (mut socket, _response) =
        connect(Url::parse(&config.skinport.websocket_url)
            .unwrap())
            .expect("Can't connect");

    println!("Connected to the server");

    // begin message loop
    loop {
        let msg = socket.read_message().expect("Error reading message");
        // println!("Received message: {}", msg);
        let msg_code = msg.to_text().unwrap().split("{").collect::<Vec<_>>()[0];
        match msg_code {
            "0" => {
                // Send response
                println!("S->C: Hello (0)");
                socket.write_message(Message::Text("40".to_string())).unwrap(); // Reply handshake (40)
                println!("C->S: ACK (40)");
                socket.write_message(Message::Text("42[\"saleFeedJoin\",{\"appid\":730,\"currency\":\"GBP\",\"locale\":\"en\"}]".to_string())).unwrap(); // Send saleFeedJoin (42)
                println!("C->S: saleFeedJoin (42)");
            },
            "2" => {
                // println!("S->C: Ping (2)");
                socket.write_message(Message::Text("3".to_string())).unwrap(); // Send pong (3)
                // println!("C->S: Pong (3)");
            },
            _ => {
                // Server sent new listing
                if msg.to_string().starts_with("42[\"saleFeed\",") {
                    println!("S->C: New listing (42)");
                    let mut trimmed_msg = msg.to_text().unwrap().replace("42[\"saleFeed\",", "");
                    trimmed_msg.pop().unwrap();

                    // Bulk of code
                    let start = Instant::now();
                    process_new_listing(trimmed_msg);
                    let duration = start.elapsed();
                    println!("Time elapsed in process_new_listing() was: {} ns / {} ms", duration.as_nanos(), (duration.as_nanos() as f64) / 1000000.0);
                }
                else {
                    // Unknown message, log it.
                    println!("Unknown message: {}", msg);
                }
            }
        }
    }
}