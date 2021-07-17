struct ComandCenterControl;
const PLUGIN_NAME: &str = "Command Center Control";

impl core::Plugin for ComandCenterControl {
    fn get_info(&self) {
        println!("Loaded: {} Plugin", PLUGIN_NAME);
    }
}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(ComandCenterControl));
}
