#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use serde_json::json;
use serde_json::{Value};
use std::vec::Vec;
use core::{Event, EventVars, Trigger, TriggerVars};

struct ProPresenter;

const PLUGIN_NAME: &str = "Pro Presenter";

// static mut events: serde_json::Value = serde_json::Value::Null;
// static mut triggers: serde_json::Value = serde_json::Value::Null;

impl core::Plugin for ProPresenter {
    fn init(&self) {
        println!("Loaded: {} Plugin", PLUGIN_NAME);

        // struct Event {
        //     type: String,
        //     name: String
        // }
        // unsafe {
        //     events = json!({
        //         "Slide Changed": [
        //             {"type": "string", "name": "Current Slide Text"},
        //             {"type": "string", "name": "Next Slide Text"},
        //             {"type": "string", "name": "Current Slide Notes"},
        //             {"type": "string", "name": "Next Slide Notes"}
        //         ]
        //     });
            
        //     triggers = json!({
        //         "Trigger Macro": [
        //             {"type": "string", "name": "Macro Name"}
        //         ],
        //         "Trigger Look": [
        //             {"type": "string", "name": "Look Name"}
        //         ],
        //         "Trigger Next Slide": [],
        //         "Clear All": [],
        //         "Clear Slide": [],
        //         "Clear Props": [],
        //         "Clear Audio": [],
        //         "Clear Video": [],
        //         "Clear Telestrator": [],
        //         "Show Stage Display Message": [{"type": "string", "name": "Message"}],
        //         "Hide Stage Display Message": []
        //     });
        // }
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
                        var_id: String::from("macro-name"),
                        var_type: String::from("string"),
                    }
                ]
            },
            Trigger {
                name: String::from("Next Slide"),
                id: String::from("next-slide"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Previous Slide"),
                id: String::from("previous-slide"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Clear All"),
                id: String::from("clear-all"),
                vars: vec![]
            },
            Trigger {
                name: String::from("clear Slide"),
                id: String::from("clear-slide"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Clear Props"),
                id: String::from("clear-props"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Clear Audio"),
                id: String::from("clear-audio"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Clear Video"),
                id: String::from("clear-video"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Clear Telestrator"),
                id: String::from("clear-telestrator"),
                vars: vec![]
            },
            Trigger {
                name: String::from("clear Logo"),
                id: String::from("clear-clogo"),
                vars: vec![]
            },
            Trigger {
                name: String::from("Show Stage Display Message"),
                id: String::from("show-stage-message"),
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
                id: String::from("hide-stage-message"),
                vars: vec![]
            }
        ];

        return triggers;
        
    }

}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(ProPresenter));
}
