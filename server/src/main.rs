use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::server::accept;

fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
                println!("Received: {}", msg);
                if msg.is_binary() || msg.is_text() {
                    let msg_copy = msg.clone();
                    websocket.write_message(msg).unwrap();
                    websocket.write_message(msg_copy).unwrap();
                }
            }
        });
    }
}
