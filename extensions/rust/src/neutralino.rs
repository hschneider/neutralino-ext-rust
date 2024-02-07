use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{io, process};
use std::io::Read;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;
use uuid::Uuid;
use crate::DEBUG;

const VERSION: &str = "1.0.5";

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    event: String,
    data: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct DataPacket {
    id: String,
    method: String,
    accessToken: String,
    data: Data,
}

pub struct Extension {
    config: Value,
    url_ipc: String,
    token: String,
    socket: Option<WebSocket<MaybeTlsStream<TcpStream>>>,
}

impl Extension {

    pub fn new() -> Self {
        //
        // Constructor

        return Extension {
            config: Value::Null,
            url_ipc: String::new(),
            token: String::new(),
            socket: None,
        };
    }

    pub fn run(&mut self, callback: fn(this: &mut Extension, d: &mut serde_json::Value)) {
        //
        // Init and run the WebSocket main loop.

        if crate::DEBUG {
            println!("Running Neutralino Extension {}", VERSION);
        }

        // Read JSON from stdin into a String
        //
        let str_json = match Self::read_stdin() {
            Ok(string) => string,
            Err(err) => {
                println!("Error reading JSON from stdin: {}", err);
                return;
            }
        };

        // Parse JSON string into a serde_json object
        //
        self.config = match serde_json::from_str(&str_json) {
            Ok(value) => value,
            Err(err) => {
                println!("Error parsing JSON: {}", err);
                return;
            }
        };

        // Get nlToken:
        //
        self.token = self.config["nlToken"].as_str().unwrap().to_string();

        // Build WebSocket URL
        //
        self.url_ipc = format!(
            "ws://127.0.0.1:{}?extensionId={}&connectToken={}",
            self.config["nlPort"].as_str().unwrap().to_string(),
            self.config["nlExtensionId"].as_str().unwrap().to_string(),
            self.config["nlConnectToken"].as_str().unwrap().to_string(),
        )
        .to_string();

        if crate::DEBUG {
            println!("WebSocket URL: {}", self.url_ipc);
        }

        // Connect to server
        //
        let (socket, _response) =
            connect(Url::parse(&self.url_ipc).unwrap()).expect("Can't connect");

        self.socket = Some(socket);

        // Main loop:
        // Listen for incoming data and trigger callback.
        //
        loop {
            let msg = self
                .socket
                .as_mut()
                .expect("Error reading socket.")
                .read()
                .expect("Error reading message");

            println!("\x1b[91mReceived: {}\x1b[0m", &msg);

            let mut d = match serde_json::from_str(&msg.to_string()) {
                Ok(value) => value,
                Err(err) => {
                    println!("Error parsing JSON: {}", err);
                    continue;
                }
            };

            // Capture app-quit events
            //
            if self.is_event(&mut d, "windowClose") || self.is_event(&mut d, "appClose") {
                if crate::DEBUG {
                    println!("ExtRust is exiting gracefully ...");
                }
                process::exit(0);
            }

            // Process IPC-packages:
            callback(self, &mut d);
        }
    }

    pub fn send_message(&mut self, event: &str, data: &str) {
        //
        // Send data to the Neutralino app and Trigger a frontend event.
        // Use this right from the callback function.

        let data = DataPacket {
            id: Uuid::new_v4().into(),
            method: "app.broadcast".into(),
            accessToken: self.token.clone().into(),
            data: Data {
                event: event.into(),
                data: data.into(),
            },
        };

        let packet = serde_json::to_string(&data).expect("Failed to serialize to JSON");
        let _res = self
            .socket
            .as_mut()
            .expect("Cannot send message.")
            .send(Message::Text(packet.clone()).into());

        if crate::DEBUG {
            println!("\x1b[32mSent: {}\x1b[0m", packet.clone());
        }
    }

    pub fn is_event(&mut self, d: &serde_json::Value, event_name: &str) -> bool {
        //
        // Check if ipc package contains a particluar eventName

        if d.get("event").is_some() {
            if d["event"].as_str().unwrap().to_string() == event_name.to_string() {
                return true;
            }
        }
        return false;
    }

    pub fn get_data(&mut self, d: &serde_json::Value) -> serde_json::Value {
        //
        // Extracts the data from the ipc package.

        if let Some(data) = d.get("data") {
            return data.clone();
        }
        return serde_json::from_str(r"{}").unwrap();
    }

    fn read_stdin() -> Result<String, io::Error> {
        //
        // Read config from stdin.

        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    }
}
