/*
    Command Center Server
    Copyright (C) 2021  Banned Patriot <realbannedpatriot@protonmail.com>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;
use libloading::Library;
use core::{Plugin, PluginRegistrar, PluginInfo, Endpoint};
use std::sync::mpsc::{Sender, SyncSender, Receiver};
use std::vec::Vec;

use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: String,
}

struct Registrar {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistrar for Registrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr, main_tx: SyncSender<String>) {
    main_tx.send(format!("Incoming TCP connection from: {}", &addr));

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");

    main_tx.send(format!("WebSocket Client Connected: {}", &addr));

    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (outgoing, incoming) = ws_stream.split();

    let notify_main_loop = incoming.try_for_each(|msg| {

        match msg.clone() {
            Message::Text(msg) => {
                main_tx.send(format!("Client {} Message: {}", &addr, &msg));
            },
            _ => {}
        }

        future::ok(())
    });

    let receive = rx.map(Ok).forward(outgoing);

    pin_mut!(notify_main_loop, receive);
    future::select(notify_main_loop, receive).await;

    main_tx.send(format!("WebSocket Client Disconnected: {}", &addr));
    peer_map.lock().unwrap().remove(&addr);
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:4444".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Bind to socket
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    let clients = state.clone();
    println!("Websocket Server Listening on: {}", addr);

    let mut registrar = Registrar {
        plugins: Vec::new(),
    };

    let cwd = env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    #[cfg(debug_assertions)]
    let plugin_path = cwd;
    #[cfg(not(debug_assertions))]
    let plugin_path = format!("{}/plugins", cwd);

    println!("# Scanning for Plugins in: {}", plugin_path);

    let paths = std::fs::read_dir(plugin_path).unwrap();
        
    let files =
    paths.filter_map(|entry| {
    entry.ok().and_then(|e|
        e.path().file_name()
        .and_then(|n| n.to_str().map(|s| format!("./{}", String::from(s))))
    )
    }).collect::<Vec<String>>();

    for file in files {
        if file.ends_with(".so") {
            println!("Found Plugin: {}", file);
            let lib = Box::leak(Box::new(Library::new(file).unwrap()));

            unsafe {
                let func: libloading::Symbol<unsafe extern "C" fn(&mut dyn PluginRegistrar) -> ()> =
                    lib.get(b"plugin_entry").unwrap();
                func(&mut registrar);
            }
        }
        
    }

    println!("# Loading Plugins\n");
    let mut not_first = false;
    let mut plugins = vec!();
    
    let mut n = 0;
    for plugin in &registrar.plugins {
        if not_first {
            println!("");
        }

        let mut plugin_info = plugin.get_info();
        plugin_info.index = n;
        plugins.push(plugin_info);
        n += 1;

        plugin.init();

        println!("\n  Events:");
        for event in plugin.get_events() {
            println!("    {}", event.name);
            for var in event.vars {
                println!("      --> {} [{}]", var.var_name, var.var_type);
            }
        }

        println!("\n  Triggers:");
        for trigger in plugin.get_triggers() {
            println!("    {}", trigger.name);
            for var in trigger.vars {
                println!("      --> {} [{}]", var.var_name, var.var_type);
            }
        }

        not_first = true;
    }

    // TODO | Debugging
    let mut endpoints: Vec<Endpoint> = vec!();
    init_endpoint(&String::from("test_endpoint"), &mut endpoints, &plugins, &registrar);
    kill_endpoint(&String::from("test_endpoint"), &endpoints, &plugins, &registrar);

    // Start Websocket Server
    let (tx_con_handler, conn_handler) = std::sync::mpsc::sync_channel(100);
    tokio::spawn( async move {
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(handle_connection(state.clone(), stream, addr, tx_con_handler.clone()));
        }
    });

    // Main Loop
    loop {
        let mut i: usize = 0;
        let mut remove_endpoints = false;
        let mut endpoints_to_remove: Vec<usize> = vec!();

        for endpoint in &endpoints {
            let msg = endpoint.rx.try_recv();
            if msg.is_ok() {
                let msg = msg.unwrap();
                match msg.as_str() {
                    "_endpoint_opened_" =>{},
                    "_endpoint_closed_" => {
                        remove_endpoints = true;
                        endpoints_to_remove.push(i);
                    },
                    _ => {
                        println!("DEBUG :: {} :: {} : {:?}", endpoint.plugin, endpoint.id, msg);
                    }
                }
            }
        }

        let msg = conn_handler.try_recv();
        if msg.is_ok() {
            let msg = msg.unwrap();
            match msg.as_str() {
                _ => {
                    println!("DEBUG :: Websocket API : {:?}", msg);

                    let peers = clients.lock().unwrap();
                    let recipients = peers.iter().map(|(_, ws_sink)| ws_sink);

                        let message  = Message::Text(format!("I got you!  ::  {}", msg));
                        for recp in recipients.clone() {
                           recp.unbounded_send(message.clone()).unwrap()
                        }

                }
            }
        }

        if remove_endpoints {
            for i in endpoints_to_remove {
                println!("Endpoint Removed: {}", endpoints[i].id);
                endpoints.remove(i);
            }
            remove_endpoints = false;
            endpoints_changed();
        }
    }

    Ok(())
}

fn init_endpoint(endpoint_id: &String, endpoints: &mut Vec<Endpoint>, plugins: &Vec<PluginInfo>, registrar: &Registrar) {
    for plugin in plugins {
        if plugin.id == "blank_plugin" {
            endpoints.push(registrar.plugins[plugin.index].init_endpoint(&endpoint_id));
            println!("Endpoint Added: {}", endpoint_id);
            endpoints_changed();
        }
    }
}

fn kill_endpoint(endpoint_id: &String, endpoints: &Vec<Endpoint>, plugins: &Vec<PluginInfo>, registrar: &Registrar) {
    for endpoint in endpoints {
        for plugin in plugins {
            if plugin.id == endpoint.plugin {
                registrar.plugins[plugin.index].kill_endpoint(&endpoint);
            }
        }
    }
}

// Function called after an endpoint is added or removed
fn endpoints_changed() {
    // TODO: Notify clients
}