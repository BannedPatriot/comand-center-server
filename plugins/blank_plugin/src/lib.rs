use std::vec::Vec;
use core::{Event, EventVars, Trigger, TriggerVars, PluginInfo, Endpoint};
use std::sync::mpsc::{Sender, Receiver};

struct ComandCenterControl;

const PLUGIN_NAME: &str = "Blank Plugin";
const PLUGIN_ID: &str = "blank_plugin";

impl core::Plugin for ComandCenterControl {
    fn init(&self) {
        println!("Loaded: {} Plugin", PLUGIN_NAME); // CHANGE
        
    }

    fn get_info(&self) -> PluginInfo {
        let mut plugin = PluginInfo::new();

        plugin.name = "Blank Plugin";
        plugin.id = "blank_plugin";

        plugin
    }

    fn get_events(&self) -> Vec<core::Event> {

        let events: Vec<Event> = vec![
            Event {
                name: "",
                id: "",
                vars: vec![
                    EventVars {
                        var_name: "",
                        var_id: "",
                        var_type: "",
                    }
                ]
            }
        ];

        events  
    }


    fn get_triggers(&self) -> Vec<core::Trigger> {

        let triggers: Vec<Trigger> = vec![
            Trigger {
                name: "",
                id: "",
                vars: vec![
                    TriggerVars {
                        var_name: "",
                        var_id: "",
                        var_type: "",
                    }
                ]
            }
        ];

        triggers
        
    }

    fn init_endpoint(&self, endpoint_id: &String) -> Endpoint {
        let (tx, main_rx) = std::sync::mpsc::channel();
        let (main_tx, rx) = std::sync::mpsc::channel();
        
        tx.send(String::from("_endpoint_opened_"));

        std::thread::spawn(move || {
            let mut running = true;
            while running {
                let msg = rx.try_recv();
                if msg.is_ok() {
                    let msg: String = msg.unwrap();
                    if msg == String::from("_term_") {
                        running = false;
                    } else {
                        println!("{} Msg Recv: {}", PLUGIN_NAME, msg);
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
