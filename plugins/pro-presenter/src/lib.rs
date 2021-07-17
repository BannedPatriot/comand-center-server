struct ProPresenter;
const PLUGIN_NAME: &str = "Pro Presenter";

impl core::Plugin for ProPresenter {
    fn get_info(&self) {
        println!("Loaded: {} Plugin", PLUGIN_NAME);
    }

}

#[no_mangle]
pub fn plugin_entry(registrar: &mut dyn core::PluginRegistrar) {
    registrar.register_plugin(Box::new(ProPresenter));
}
