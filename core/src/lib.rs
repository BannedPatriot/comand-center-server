use std::vec::Vec;
use std::sync::mpsc::{Sender};

#[derive(Debug)]
pub struct Event {
    pub name: &'static str,
    pub id: &'static str,
    pub vars: Vec<EventVars>
}

#[derive(Debug)]
pub struct EventVars {
    pub var_name: &'static str,
    pub var_id: &'static str,
    pub var_type: &'static str
}

#[derive(Debug)]
pub struct Trigger {
    pub name: &'static str,
    pub id: &'static str,
    pub vars: Vec<TriggerVars>
}

#[derive(Debug)]
pub struct TriggerVars {
    pub var_name: &'static str,
    pub var_id: &'static str,
    pub var_type: &'static str
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
    fn init_endpoint(&self, main_tx: Sender<String>) -> Sender<String>; // bring up an endpoint
    fn kill_endpoint(&self, endpoint: &Sender<String>); // terminate an endpoint

    fn get_events(&self) -> Vec<Event>; // get list of possible events to trigger macros
    fn get_triggers(&self) -> Vec<Trigger>; // get list of methords that can be triggered

    fn trigger(&self, method: &str, vars: String); // trigger event on endpoint
}
