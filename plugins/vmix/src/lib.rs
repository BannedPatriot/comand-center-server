struct Vmix;
const PLUGIN_NAME: &str = "vMix";

impl core::Plugin for Vmix {
    fn init(&self) {
        println!("Loaded: {} Plugin", PLUGIN_NAME);
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(Vmix));
}
