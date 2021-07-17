use std::vec::Vec;
use core::{Event, EventVars, Trigger, TriggerVars};
use std::sync::mpsc::{Sender, Receiver};

struct ComandCenterControl;

const PLUGIN_NAME: &str = "Command Center Control";

impl core::Plugin for ComandCenterControl {
    fn init(&self) {
        println!("Loaded: {} Plugin", PLUGIN_NAME);
        
    }

    fn get_events(&self) -> Vec<core::Event> {

        let events: Vec<Event> = vec![
            Event {
                name: String::from("Key Pressed"),
                id: String::from("key-pressed"),
                vars: vec![
                    EventVars {
                        var_name: String::from("Key Identifier"),
                        var_id: String::from("key-identifier"),
                        var_type: String::from("string"),
                    }
                ]
            }
        ];

        return events;
        
    }


    fn get_triggers(&self) -> Vec<core::Trigger> {

        let triggers: Vec<Trigger> = vec![
            Trigger {
                name: String::from(""),
                id: String::from(""),
                vars: vec![]
            }
        ];

        return triggers;
        
    }

    fn init_endpoint(&self, main_tx: Sender<String>) -> Sender<String> {
        let mut message: String = "".to_owned();
            message = message + PLUGIN_NAME;
            message = message + ": Endpoint Opened";
            main_tx.send(String::from(message));

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
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            let mut message: String = "".to_owned();
            message = message + PLUGIN_NAME;
            message = message + ": Endpoint Closed";
            main_tx.send(String::from(message));
        });

        return thread_tx;
    }

    fn kill_endpoint(&self, endpoint: &Sender<String>) {
        endpoint.send(String::from("_term_"));
    }

    fn trigger(&self, method: String, vars: String) {
        // todo
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(ComandCenterControl));
}
