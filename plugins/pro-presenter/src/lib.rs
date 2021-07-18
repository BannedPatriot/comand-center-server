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
                name: "Slide Changed",
                id: "slide-changed",
                vars: vec![
                    EventVars {
                        var_name: "Current Slide Text",
                        var_id: "current-slide-text",
                        var_type: "string",
                    },
                    EventVars {
                        var_name: "Next Slide Text",
                        var_id: "next-slide-text",
                        var_type: "string",
                    },
                    EventVars {
                        var_name: "Current Slide Notes",
                        var_id: "current-slide-notes",
                        var_type: "string",
                    },
                    EventVars {
                        var_name: "Next Slide Notes",
                        var_id: "next-slide-notes",
                        var_type: "string",
                    }
                ]
            },
            Event {
                name: "Stage Display Layout Changed",
                id: "stage-layout-changed",
                vars: vec![
                    EventVars {
                        var_name: "name",
                        var_id: "id",
                        var_type: "type",
                    }
                ]
            }
        ];

        return events;
        
    }


    fn get_triggers(&self) -> Vec<core::Trigger> {

        let triggers: Vec<Trigger> = vec![
            Trigger {
                name: "Macro",
                id: "macro",
                vars: vec![
                    TriggerVars {
                        var_name: "Macro Name",
                        var_id: "macro_name",
                        var_type: "string",
                    }
                ]
            },
            Trigger {
                name: "Next Slide",
                id: "next_slide",
                vars: vec![]
            },
            Trigger {
                name: "Previous Slide",
                id: "previous_slide",
                vars: vec![]
            },
            Trigger {
                name: "Clear All",
                id: "clear_all",
                vars: vec![]
            },
            Trigger {
                name: "clear Slide",
                id: "clear_slide",
                vars: vec![]
            },
            Trigger {
                name: "Clear Props",
                id: "clear_props",
                vars: vec![]
            },
            Trigger {
                name: "Clear Audio",
                id: "clear_audio",
                vars: vec![]
            },
            Trigger {
                name: "Clear Video",
                id: "clear_video",
                vars: vec![]
            },
            Trigger {
                name: "Clear Telestrator",
                id: "clear_telestrator",
                vars: vec![]
            },
            Trigger {
                name: "clear Logo",
                id: "clear_logo",
                vars: vec![]
            },
            Trigger {
                name: "Show Stage Display Message",
                id: "show_stage_message",
                vars: vec![
                    TriggerVars {
                        var_name: "Message",
                        var_id: "message",
                        var_type: "string",
                    }
                ]
            },
            Trigger {
                name: "Hide Stage Display Message",
                id: "hide_stage_message",
                vars: vec![]
            }
        ];

        return triggers;
        
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
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            main_tx.send(format!("{} - Endpoint Closed", PLUGIN_NAME));
        });

        return thread_tx;
    }

    fn kill_endpoint(&self, endpoint: &Sender<String>) {
        endpoint.send(String::from("_term_"));
    }

    fn trigger(&self, method: &str, vars: String) {
        // todo
    }



}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(ProPresenter));
}
