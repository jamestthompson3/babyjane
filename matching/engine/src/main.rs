extern crate env_logger;
extern crate ws;

use ws::connect;
use ws::Message;

fn main() {
    // Setup logging
    env_logger::init();

    // Connect to the url and call the closure
    connect("ws://127.0.0.1:8000", |out| {
        // The handler needs to take ownership of out, so we use move
        move |msg: Message| {
            let is_text = msg.is_text();
            println!("Classification of msg {}", is_text);
            if is_text {
                println!("message receieved as text, {}", msg);
            } else {
                println!("message receieved, {}", msg);
                out.send("Invalid data format").unwrap();
            }
            Ok(())
        }
        // Close the connection
        // out.close(CloseCode::Normal)
    }).unwrap();
}
