
use std::sync::Arc;
use std::sync::RwLock;
use std::collections::HashMap;
use std::net::UdpSocket;

use tokio::runtime::Runtime;

//need a server using tokio 
#[tokio::main]
async fn main() {
    println!("Creating server...");
    let runtime = Runtime::new().unwrap();
    let addr = "127.0.0.1:8080";
    let socket = UdpSocket::bind(addr).expect("Failed to bind to address");

    let server = std::net::UdpSocket::bind(addr);
    println!("Server initiated!");

    println!("Server listening on: {}", addr);

    // Spawn an asynchronous task using executor
    runtime.spawn(async {
            println!("Asynchronous task is running!");
        });
    //Storing ids in a hashmap
    //let connections = Arc::new(RwLock::new(HashMap::new()));
    //hashmap in id:entity state 
    //let entities = Arc::new(RwLock::new(HashMap::new()));
    //new player new id in prev entities hash
    let mut buf = [0; 1024];

    //let counter = Arc::new(RwLock::new(0))
    loop {
        // Receive data from a client
        let (n, client_addr) = socket.recv_from(&mut buf).expect("Failed to receive data");

        // Process the received data (modify as needed)
        let received_data = &buf[..n];
        println!("Received data from {}: {:?}", client_addr, received_data);

        // Echo the data back to the client
        socket.send_to(received_data, &client_addr).expect("Failed to send data back");
    };
}

