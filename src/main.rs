/*
    Command Center Server
    Copyright (C) 2021  Banned Patriot <realbannedpatriot@protonmail.com>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use libloading::Library;
use core::{Plugin, PluginRegistrar};
use std::env;
use std::vec::Vec;

struct Registrar {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginRegistrar for Registrar {
    fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }
}

fn main() {
    let mut registrar = Registrar {
        plugins: Vec::new(),
    };

    let cwd = env::current_dir().unwrap().as_os_str().to_str().unwrap().to_string();
    #[cfg(debug_assertions)]
    let plugin_path = cwd;
    #[cfg(not(debug_assertions))]
    let plugin_path = format!("{}/plugins", cwd);

    println!("# Scanning for Plugins in: {}", plugin_path);

    let paths = std::fs::read_dir(plugin_path).unwrap();
        
    let files =
    paths.filter_map(|entry| {
    entry.ok().and_then(|e|
        e.path().file_name()
        .and_then(|n| n.to_str().map(|s| format!("./{}", String::from(s))))
    )
    }).collect::<Vec<String>>();

    for file in files {
        if file.ends_with(".so") {
            println!("Found Plugin: {}", file);
            let lib = Box::leak(Box::new(Library::new(file).unwrap()));

            unsafe {
                let func: libloading::Symbol<unsafe extern "C" fn(&mut dyn PluginRegistrar) -> ()> =
                    lib.get(b"plugin_entry").unwrap();
                func(&mut registrar);
            }
        }
        
    }

    println!("# Loading Plugins\n");
    let mut not_first = false;
    let plugins = registrar.plugins;

    for plugin in &plugins {
        if not_first {
            println!("");
        }
        
        plugin.init();

        println!("\n  Events:");
        for event in plugin.get_events() {
            println!("    {}", event.name);
            for var in event.vars {
                println!("      --> {} [{}]", var.var_name, var.var_type);
            }
        }

        println!("\n  Triggers:");
        for trigger in plugin.get_triggers() {
            println!("    {}", trigger.name);
            for var in trigger.vars {
                println!("      --> {} [{}]", var.var_name, var.var_type);
            }
        }

        plugin.trigger(String::from("clear-all"), String::from("json"));
        not_first = true;
    }

    let (main_tx, main_rx) = std::sync::mpsc::channel();

    let endpoint_test = plugins[0].init_endpoint(main_tx.clone());
    
    loop {
        let msg = main_rx.try_recv();
        if !msg.is_err() {
            // Got msg, do something
            println!("Log: {}", msg.unwrap());
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
