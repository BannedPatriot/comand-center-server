extern crate derive_new;
use std::vec::Vec;
use std::sync::mpsc::{Sender,Receiver};
use derive_new::new;

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

#[derive(new, Debug)]
pub struct PluginInfo {
    #[new(default)]
    pub name: &'static str,
    #[new(default)]
    pub id: &'static str,
    #[new(default)]
    pub index: usize
}

#[derive(Debug)]
pub struct Endpoint {
    pub id: String,
    pub rx: Receiver<String>,
    pub tx: Sender<String>,
    pub plugin: &'static str
}

pub trait PluginRegistrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>);
}

pub trait Plugin {
    fn init(&self); // Log data about plugin
    fn get_info(&self) -> PluginInfo;

    // fn get_options(&self); // get configuration options for endpoint

    // /*  Endpoints are devices / software applications that interface with
    //     comand center through their respective plugins. Plugins handle
    //     the logisticts of opening and mantinaing these connections.
    // */
    fn init_endpoint(&self, id: &String) -> Endpoint; // bring up an endpoint
    fn kill_endpoint(&self, endpoint: &Endpoint); // terminate an endpoint

    fn get_events(&self) -> Vec<Event>; // get list of possible events to trigger macros
    fn get_triggers(&self) -> Vec<Trigger>; // get list of methords that can be triggered

    fn trigger(&self, method: &str, vars: String); // trigger event on endpoint
}
