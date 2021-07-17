use std::vec::Vec;
use core::{Event, EventVars, Trigger, TriggerVars};
use std::sync::mpsc::{Sender, Receiver};

struct ProPresenter;

const PLUGIN_NAME: &str = "Pro Presenter";

impl core::Plugin for ProPresenter {
    fn init(&self) {
        println!("Loaded: {} Plugin", PLUGIN_NAME);
    }

    fn get_events(&self) -> Vec<core::Event> {

        let events: Vec<Event> = vec![
            Event {
                name: String::from("Slide Changed"),
                id: String::from("slide-changed"),
                vars: vec![
                    EventVars {
                        var_name: String::from("Current Slide Text"),
                        var_id: String::from("current-slide-text"),
                        var_type: String::from("string"),
                    },
                    EventVars {
                        var_name: String::from("Next Slide Text"),
                        var_id: String::from("next-slide-text"),
                        var_type: String::from("string"),
                    },
                    EventVars {
                        var_name: String::from("Current Slide Notes"),
                        var_id: String::from("current-slide-notes"),
                        var_type: String::from("string"),
                    },
                    EventVars {
                        var_name: String::from("Next Slide Notes"),
                        var_id: String::from("next-slide-notes"),
                        var_type: String::from("string"),
                    }
                ]
            },
            Event {
                name: String::from("Stage Display Layout Changed"),
                id: String::from("stage-layout-changed"),
                vars: vec![
                    EventVars {
                        var_name: String::from("name"),
                        var_id: String::from("id"),
                        var_type: String::from("type"),
                    }
                ]
            }
        ];

        return events;
        
    }


    fn get_triggers(&self) -> Vec<core::Trigger> {

        let triggers: Vec<Trigger> = vec![
            Trigger {
                name: String::from("Macro"),
                id: String::from("macro"),
                vars: vec![
                    TriggerVars {
                        var_name: String::from("Macro Name"),
                        var_id: String::from("macro_name"),
                        var_type: String::from("string"),
                    }
                ]
            },
            Trigger {
                name: String::from("Next Slide"),
                id: String::from("next_slide"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Previous Slide"),
                id: String::from("previous_slide"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Clear All"),
                id: String::from("clear_all"),
                vars: vec![]
            },
            Trigger {
                name: String::from("clear Slide"),
                id: String::from("clear_slide"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Clear Props"),
                id: String::from("clear_props"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Clear Audio"),
                id: String::from("clear_audio"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Clear Video"),
                id: String::from("clear_video"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Clear Telestrator"),
                id: String::from("clear_telestrator"),
                vars: vec![]
            },
            Trigger {
                name: String::from("clear Logo"),
                id: String::from("clear_logo"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Show Stage Display Message"),
                id: String::from("show_stage_message"),
                vars: vec![
                    TriggerVars {
                        var_name: String::from("Message"),
                        var_id: String::from("message"),
                        var_type: String::from("string"),
                    }
                ]
            },
            Trigger {
                name: String::from("Hide Stage Display Message"),
                id: String::from("hide_stage_message"),
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
    registrar.register_plugin(Box::new(ProPresenter));
}
