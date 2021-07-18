use std::vec::Vec;
use core::{Event, EventVars, Trigger, TriggerVars};
use std::sync::mpsc::{Sender, Receiver};

struct ComandCenterControl;

const PLUGIN_NAME: &str = "Blank Plugin";

impl core::Plugin for ComandCenterControl {
    fn init(&self) {
        println!("Loaded: {} Plugin", PLUGIN_NAME); // CHANGE
        
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

    fn init_endpoint(&self, main_tx: Sender<String>) -> Sender<String> {
        main_tx.send(format!("{} - Endpoint Opened", PLUGIN_NAME));

        let (thread_tx, thread_rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            let mut running = true;
            while running {
                let msg = thread_rx.try_recv();
                if msg.is_ok() {
                    let msg = msg.unwrap();
                    if msg == String::from("_term_") {
                        running = false;
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100)); // REMOVE
            }
            main_tx.send(format!("{} - Endpoint Closed", PLUGIN_NAME));
        });

        return thread_tx;
    }

    fn kill_endpoint(&self, endpoint: &Sender<String>) {
        endpoint.send(String::from("_term_"));
    }

    fn trigger(&self, method: &str, vars: String) {
        // TODO
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(ComandCenterControl));
}
