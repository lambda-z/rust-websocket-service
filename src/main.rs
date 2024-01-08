use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use tungstenite::Message;

// reading & writing & coding & music & movies & games & sports & travel & food & sleep & ...
/// A WebSocket echo server
///
/*
#### 代码说明
在这段代码中，|| 符号表示一个闭包（closure），它用于创建一个匿名函数。闭包是一种特殊的函数，可以捕获其环境中的变量，并在需要时使用它们。
在这个例子中，move || { ... } 表示创建一个不接受任何参数的闭包。move 关键字告诉编译器闭包需要获取其环境中的所有权，并将其移动到闭包内部，以便在闭包执行时可以使用这些变量。
闭包被传递给 spawn 函数，用于在新的线程中运行。每当服务器接收到一个新的连接时，都会为该连接创建一个新的线程，并在该线程中执行闭包中的代码。
需要注意的是，这段代码是用 Rust 编程语言编写的。它使用了 Rust 标准库中的 TcpListener 和 websocket 相关的功能。
*/
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
