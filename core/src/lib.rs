extern crate derive_new;
use std::vec::Vec;
use std::sync::mpsc::{Sender,Receiver};
use derive_new::new;
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AppConfig {
    pub endpoints: Vec<ConfigEndpoint>,
    pub macros: Vec<ConfigMacro>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigEndpoint {
    pub name: String,
    pub plugin: String,
    pub enabled: bool,
    pub options: Vec<ConfigOptions>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigOptions {
    pub option: String,
    pub value: String
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigMacro {
    pub name: String,
    pub enabled: bool,
    pub events: Vec<ConfigMacroEvent>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigMacroEvent {
    pub endpoint: String,
    pub event: String,
    pub logic: Vec<ConfigMacroFunction>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigMacroFunction {
    pub function: String,
    pub options: Vec<ConfigMacroFunctionOptions>,
    pub logic: Vec<ConfigMacroFunction>
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigMacroFunctionOptions {
    pub option: String,
    pub value: String
}


#[derive(Debug)]
pub struct Event {
    pub name: &'static str,
    pub id: &'static str,
    pub vars: Vec<EventVar>
}

#[derive(Debug)]
pub struct EventVar {
    pub var_name: &'static str,
    pub var_id: &'static str,
    pub var_type: &'static str
}

#[derive(Debug)]
pub struct Trigger {
    pub name: &'static str,
    pub id: &'static str,
    pub vars: Vec<TriggerVar>
}

#[derive(Debug)]
pub struct TriggerVar {
    pub var_name: &'static str,
    pub var_id: &'static str,
    pub var_type: &'static str
}

#[derive(Clone, Debug)]
pub struct PluginInfo {
    pub index: usize,
    pub name: &'static str,
    pub id: &'static str,
    pub version: &'static str,
    pub options: Vec<PluginOption>
}

#[derive(Clone, Debug)]
pub struct PluginOption {
    pub name: &'static str,
    pub option_type: &'static str,
    pub option_list: Vec<&'static str>,
    pub option_default: &'static str,
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
