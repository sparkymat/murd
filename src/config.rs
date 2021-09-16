use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub listen_host: String,
    pub listen_port: u32,
}

impl Config {
    pub fn listen_address(&self) -> String {
        format!("{}:{}", self.listen_host, self.listen_port)
    }
}

pub fn load() -> Config {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("config")).unwrap();

    // Print out our settings (as a HashMap)
    return settings.try_into::<Config>().unwrap();
}
