use std::vec::Vec;
use core::{*};
use std::sync::mpsc::{Sender, Receiver};

struct ComandCenterControl;

impl core::Plugin for ComandCenterControl {
    fn init(&self) {
        println!("Loaded: {} Plugin", self.get_info().name); // CHANGE
        
    }

    fn get_info(&self) -> PluginInfo {
        PluginInfo {
            index: 0, // Updated at runtime, but CCS needs a default value.
            name: "Example Plugin", // Plugin Name
            id: "example-plugin", // Unique Identifer, no spaces.
            version: "0.1.0", // Plugin Version

            // Endpoint Configuration Options
            options: vec!(
                PluginOption {
                    name: "Version",
                    option_type: "text",
                    option_list: vec!(
                        "Pro Presenter 7",
                        "Pro Presenter 6"
                    ),
                    option_default: "Pro Presenter 7"
                },
                PluginOption {
                    name: "IP Address",
                    option_type: "text",
                    option_list: vec!(),
                    option_default: "127.0.0.1"
                },
                PluginOption {
                    name: "Port",
                    option_type: "text",
                    option_list: vec!(),
                    option_default: "1111"
                },
                PluginOption {
                    name: "Remote Password",
                    option_type: "text",
                    option_list: vec!(),
                    option_default: ""
                },
                PluginOption {
                    name: "Stage Display Password",
                    option_type: "text",
                    option_list: vec!(),
                    option_default: ""
                }
            )

        }
    }

    // Return plugin events
    fn get_events(&self) -> Vec<Event> { vec!(
        Event {
            name: "",
            id: "",
            vars: vec!(
                EventVar {
                    var_name: "",
                    var_id: "",
                    var_type: ""
                }
            )
        },
        Event {
            name: "",
            id: "",
            vars: vec!(
                EventVar {
                    var_name: "",
                    var_id: "",
                    var_type: ""
                }
            )
        }
    )}

    // Return plugin triggers
    fn get_triggers(&self) -> Vec<Trigger> { vec!(
        Trigger {
            name: "",
            id: "",
            vars: vec!(
                TriggerVar {
                    var_name: "",
                    var_id: "",
                    var_type: ""
                }
            )
        }
    )}

    fn init_endpoint(&self, endpoint_id: &String) -> Endpoint {
        let (tx, main_rx) = std::sync::mpsc::channel();
        let (main_tx, rx) = std::sync::mpsc::channel();
        tx.send(String::from("_endpoint_opened_"));

        let info = self.get_info();
        let events = self.get_events();
        let triggers = self.get_triggers();
        std::thread::spawn(move || {
            let mut running = true;
            while running {
                let msg = rx.try_recv();
                if msg.is_ok() {
                    let msg: String = msg.unwrap();
                    if msg == String::from("_term_") {
                        running = false;
                    } else {
                        println!("{} ({}) Msg Recv: {}", info.name, info.version, msg);
                        
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(500)); // REMOVE

            }
            tx.send(String::from("_endpoint_closed_"));
        });

        Endpoint {
            id: endpoint_id.clone(),
            rx: main_rx,
            tx: main_tx,
            plugin: self.get_info().id
        }
    }

    fn kill_endpoint(&self, endpoint: &Endpoint) {
        endpoint.tx.send(String::from("_term_"));
    }

    fn trigger(&self, method: &str, vars: String) {
        // TODO
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(ComandCenterControl));
}
