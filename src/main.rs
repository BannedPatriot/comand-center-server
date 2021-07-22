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
extern crate yaml_rust;
use serde::__private::de::borrow_cow_bytes;
use yaml_rust::{YamlLoader, YamlEmitter};
use yaml_rust::Yaml;
use linked_hash_map::LinkedHashMap;

use futures_channel::mpsc::{unbounded, UnboundedSender};
use futures_util::{future, pin_mut, stream::TryStreamExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tungstenite::protocol::Message;
use libloading::Library;
use core::*;
use std::borrow::BorrowMut;
use std::hash::Hash;
use std::sync::mpsc::{Sender, SyncSender, Receiver};
use std::vec::Vec;
use std::fs::File;
use std::io::prelude::*;


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

struct AppRuntime {
    config: Yaml,
    endpoints: Vec<Endpoint>,
    registrar: Registrar,
    plugins: Vec<PluginInfo>
}



#[tokio::main]
async fn main() -> Result<(), IoError> {

    // Read Config File
    let mut file = File::open("config.yaml").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);

    // Get Mutable 'endpoints'
    let mut docs = YamlLoader::load_from_str(&contents).unwrap();
    let mut config = docs[0].to_owned();

    let mut app_runtime = AppRuntime {
        config: config,
        endpoints: vec!(),
        registrar:  Registrar { plugins: vec!() },
        plugins: vec!(),
    };




    // if let Yaml::Hash(hash_map) = config {
    //     if let Yaml::Array(endpoints) = hash_map[&Yaml::from_str("endpoints")].borrow_mut() {
    //         for endpoint in endpoints.iter_mut() {
    //             if let Yaml::Hash(endpoint) = endpoint.borrow_mut() {
    //                 if endpoint[&Yaml::from_str("name")] == Yaml::from_str("Current Name") {
    //                     endpoint.insert(Yaml::from_str("name"), Yaml::from_str("New Name"));
    //                 }
    //             }
    //         }
    //     }
    // }

    // let yaml = serde_yaml::to_string(&config);
    // let mut file = File::create("config.yaml")?;
    // file.write_all(&yaml.unwrap().as_bytes())?;


    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:4444".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Bind to socket
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    let clients = state.clone();
    println!("Websocket Server Listening on: {}", addr);

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
            unsafe {
                let lib = Box::leak(Box::new(Library::new(file).unwrap()));
                let func: libloading::Symbol<unsafe extern "C" fn(&mut dyn PluginRegistrar) -> ()> =
                    lib.get(b"plugin_entry").unwrap();
                func(&mut app_runtime.registrar);
            }
        }
        
    }

    println!("# Loading Plugins\n");
    let mut not_first = false;
    
    let mut n = 0;
    for plugin in &app_runtime.registrar.plugins {
        if not_first {
            println!("");
        }

        let mut plugin_info = plugin.get_info();
        plugin_info.index = n;
        app_runtime.plugins.push(plugin_info);
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
    init_endpoint(&mut app_runtime, &String::from("test_endpoint"));
    kill_endpoint(&mut app_runtime, &String::from("test_endpoint"));

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

        for endpoint in &app_runtime.endpoints {
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
                println!("Endpoint Removed: {}", app_runtime.endpoints[i].id);
                app_runtime.endpoints.remove(i);
            }
            remove_endpoints = false;
            endpoints_changed();
        }
    }

    

    Ok(())
}


fn init_endpoint(app_runtime: &mut AppRuntime, endpoint_id: &String) {
    for plugin in &app_runtime.plugins {
        if plugin.id == "example-plugin" {

            app_runtime.endpoints.push(app_runtime.registrar.plugins[plugin.index].init_endpoint(&endpoint_id));
            println!("Endpoint Added: {}", endpoint_id);
            endpoints_changed();
        }
    }
}

fn kill_endpoint(app_runtime: &AppRuntime, endpoint_id: &String) {
    for endpoint in &app_runtime.endpoints {
        for plugin in &app_runtime.plugins {
            if plugin.id == endpoint.plugin {
                app_runtime.registrar.plugins[plugin.index].kill_endpoint(&endpoint);
            }
        }
    }
}

// Function called after an endpoint is added or removed
fn endpoints_changed() {
    // TODO: Notify clients
}





