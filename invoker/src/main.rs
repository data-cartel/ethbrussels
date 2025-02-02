use near_jsonrpc_client::{methods, JsonRpcClient};
use near_primitives::types::{BlockReference, Finality};

// const CONNECTION: &'static str = "wss://events.near.stream/ws";

#[tokio::main]
// v2.ref-finance.near
async fn main() -> anyhow::Result<()> {
    let client = JsonRpcClient::connect("https://rpc.testnet.near.org");

    let request = methods::block::RpcBlockRequest {
        block_reference: BlockReference::Finality(Finality::Final),
    };

    let server_status = client.call(request).await?;

    println!("{:?}", server_status);

    // let (mut receiver, mut sender) = client.split().unwrap();

    // let (tx, rx) = channel();

    // let tx_1 = tx.clone();

    // let send_loop = thread::spawn(move || {
    //     loop {
    //         // Send loop
    //         let message = match rx.recv() {
    //             Ok(m) => m,
    //             Err(e) => {
    //                 println!("Send Loop: {:?}", e);
    //                 return;
    //             }
    //         };
    //         match message {
    //             OwnedMessage::Close(_) => {
    //                 let _ = sender.send_message(&message);
    //                 // If it's a close message, just send it and then return.
    //                 return;
    //             }
    //             _ => (),
    //         }
    //         // Send the message
    //         match sender.send_message(&message) {
    //             Ok(()) => (),
    //             Err(e) => {
    //                 println!("Send Loop: {:?}", e);
    //                 let _ = sender.send_message(&Message::close());
    //                 return;
    //             }
    //         }
    //     }
    // });

    // let receive_loop = thread::spawn(move || {
    //     // Receive loop
    //     for message in receiver.incoming_messages() {
    //         let message = match message {
    //             Ok(m) => m,
    //             Err(e) => {
    //                 println!("Receive Loop: {:?}", e);
    //                 let _ = tx_1.send(OwnedMessage::Close(None));
    //                 return;
    //             }
    //         };
    //         match message {
    //             OwnedMessage::Close(_) => {
    //                 // Got a close message, so send a close message and return
    //                 let _ = tx_1.send(OwnedMessage::Close(None));
    //                 return;
    //             }
    //             OwnedMessage::Ping(data) => {
    //                 match tx_1.send(OwnedMessage::Pong(data)) {
    //                     // Send a pong in response
    //                     Ok(()) => (),
    //                     Err(e) => {
    //                         println!("Receive Loop: {:?}", e);
    //                         return;
    //                     }
    //                 }
    //             }
    //             // Say what we received
    //             _ => println!("Receive Loop: {:?}", message),
    //         }
    //     }
    // });

    // loop {
    //     let mut input = String::new();

    //     stdin().read_line(&mut input).unwrap();

    //     let trimmed = input.trim();

    //     let message = match trimmed {
    //         "/close" => {
    //             // Close the connection
    //             let _ = tx.send(OwnedMessage::Close(None));
    //             break;
    //         }
    //         // Send a ping
    //         "/ping" => OwnedMessage::Ping(b"PING".to_vec()),
    //         // Otherwise, just send text
    //         _ => OwnedMessage::Text(trimmed.to_string()),
    //     };

    //     match tx.send(message) {
    //         Ok(()) => (),
    //         Err(e) => {
    //             println!("Main Loop: {:?}", e);
    //             break;
    //         }
    //     }
    // }

    // // We're exiting

    // println!("Waiting for child threads to exit");

    // let _ = send_loop.join();
    // let _ = receive_loop.join();

    // println!("Exited");

    Ok(())
}
