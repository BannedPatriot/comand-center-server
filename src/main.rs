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

    println!("Searching for Plugins in: {}", plugin_path);

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

    for plugin in registrar.plugins {
        plugin.get_info();
    }
}
