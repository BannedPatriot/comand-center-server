struct Onyx;
const PLUGIN_NAME: &str = "Onyx";

impl core::Plugin for Onyx {
    fn init(&self) {
        println!("Loaded: {} Plugin", PLUGIN_NAME);
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(Onyx));
}
