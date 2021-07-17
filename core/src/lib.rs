#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
use std::vec::Vec;

use serde_json::{Value};

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub id: String,
    pub vars: Vec<EventVars>
}

#[derive(Debug)]
pub struct EventVars {
    pub var_name: String,
    pub var_id: String,
    pub var_type: String
}

#[derive(Debug)]
pub struct Trigger {
    pub name: String,
    pub id: String,
    pub vars: Vec<TriggerVars>
}

#[derive(Debug)]
pub struct TriggerVars {
    pub var_name: String,
    pub var_id: String,
    pub var_type: String
}

pub trait PluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>);
}

pub trait Plugin {
    fn init(&self); // Log data about plugin

    // fn get_options(&self); // get configuration options for endpoint

    // /*  Endpoints are devices / software applications that interface with
    //     comand center through their respective plugins. Plugins handle
    //     the logisticts of opening and mantinaing these connections.
    // */
    // fn init_endpoint(&self); // bring up an endpoint
    // fn kill_endpoint(&self); // terminate an endpoint

    fn get_events(&self) -> Vec<Event>; // get list of possible events to trigger macros
    fn get_triggers(&self) -> Vec<Trigger>; // get list of methords that can be triggered

    // fn trigger(&self); // trigger event on endpoint
}
