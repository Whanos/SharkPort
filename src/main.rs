#![allow(dead_code)]

mod config;
mod parse_json;
mod item_processor;

use tungstenite::{connect, Message};
use url::Url;
use crate::item_processor::process_new_listing;
use std::time::Instant;

macro_rules! debug_println {
    ($($arg:tt)*) => (if ::std::cfg!(debug_assertions) { ::std::println!($($arg)*); })
}

fn main() {
    println!("SharkPort");
    let config = config::read_config();

    let (mut socket, _response) =
        connect(Url::parse(&config.skinport.websocket_url)
            .unwrap())
            .expect("Can't connect");

    debug_println!("Connected to the server");

    // begin message loop
    loop {
        let msg = socket.read_message().expect("Error reading message");
        // println!("Received message: {}", msg);
        let msg_code = msg.to_text().unwrap().split("{").collect::<Vec<_>>()[0];
        match msg_code {
            "0" => {
                // Send response
                debug_println!("S->C: Hello (0)");
                socket.write_message(Message::Text("40".to_string())).unwrap(); // Reply handshake (40)
                debug_println!("C->S: ACK (40)");
                socket.write_message(Message::Text("42[\"saleFeedJoin\",{\"appid\":730,\"currency\":\"GBP\",\"locale\":\"en\"}]".to_string())).unwrap(); // Send saleFeedJoin (42)
                debug_println!("C->S: saleFeedJoin (42)");
            },
            "2" => {
                debug_println!("S->C: Ping (2)");
                socket.write_message(Message::Text("3".to_string())).unwrap(); // Send pong (3)
                debug_println!("C->S: Pong (3)");
            },
            _ => {
                // Server sent new listing
                if msg.to_string().starts_with("42[\"saleFeed\",") {
                    debug_println!("S->C: New listing (42)");
                    debug_println!("{}", msg.to_string());
                    let mut trimmed_msg = msg.to_text().unwrap().replace("42[\"saleFeed\",", "");
                    trimmed_msg.pop().unwrap();

                    // Bulk of code
                    let start = Instant::now();
                    process_new_listing(trimmed_msg);
                    let duration = start.elapsed();
                    debug_println!("Time elapsed in process_new_listing() was: {} ns / {} ms", duration.as_nanos(), (duration.as_nanos() as f64) / 1000000.0);
                }
                else {
                    // Unknown message, log it.
                    debug_println!("Unknown message: {}", msg);
                }
            }
        }
    }
}