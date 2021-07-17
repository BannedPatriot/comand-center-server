struct LanBox;
const PLUGIN_NAME: &str = "LanBox";

impl core::Plugin for LanBox {
    fn get_info(&self) {
        println!("Loaded: {} Plugin", PLUGIN_NAME);
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(LanBox));
}
