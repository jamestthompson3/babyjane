extern crate ws;
extern crate env_logger;

use ws::{connect, CloseCode};


fn main () {

    // Setup logging
    env_logger::init();

    // Connect to the url and call the closure
    if let Err(error) = connect("ws://127.0.0.1:8000", |out| {

        // Queue a message to be sent when the WebSocket is open
        if let Err(_) = out.send("Hello WebSocket") {
            println!("Websocket couldn't queue an initial message.")
        } else {
            println!("Client sent message 'Hello WebSocket'. ")
        }

        // The handler needs to take ownership of out, so we use move
        move |msg| {
            // match msg {
            // Messages::Buy => println!("Client got buy message '{}'. ", msg),
            // Messages::Sell => println!("Client got sell message '{}'. ", msg)
            // }

            // Handle messages received on this connection
           println!("Client got message '{}'. ", msg);

            // Close the connection
            out.close(CloseCode::Normal)
        }

    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to: {:?}", error);
    }

}
