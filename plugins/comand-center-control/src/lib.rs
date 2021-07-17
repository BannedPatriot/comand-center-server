use std::vec::Vec;
use core::{Event, EventVars, Trigger, TriggerVars};

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
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(ComandCenterControl));
}
