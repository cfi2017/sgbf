use onesignal_rust_api::apis::configuration::{ApiKey, Configuration};
use crate::config::OneSignal;

pub fn create_onesignal_configuration(config: &OneSignal) -> Option<Configuration> {
    let key = config.key.as_ref()?;
    let mut configuration = Configuration::new();
    configuration.app_key_token = Some(key.to_owned());
    Some(configuration)
}

