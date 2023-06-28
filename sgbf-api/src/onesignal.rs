use onesignal_rust_api::apis::configuration::{ApiKey, Configuration};
use crate::config::OneSignal;

pub fn create_onesignal_configuration(config: &OneSignal) -> Option<Configuration> {
    let key = config.key.as_ref()?;
    let mut configuration = Configuration::new();
    configuration.api_key = Some(ApiKey {
        prefix: Some("Basic".to_string()),
        key: key.clone(),
    });
    Some(configuration)
}

