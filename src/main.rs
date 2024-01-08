use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use tungstenite::Message;

// reading & writing & coding & music & movies & games & sports & travel & food & sleep & ...
/// A WebSocket echo server
fn main () {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
        // println!("New connection: {}", stream.unwrap().peer_addr().unwrap());
        spawn (move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read().unwrap();
                println!("Received: {}", msg);
                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    //  返回当前时间
                    let msg = format!("{}: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
                    websocket.send(Message::from(msg)).unwrap();
                }
            }
        });
    }
}
