use crate::method::Method;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{OnceCell, RwLock};
use toml::Table;

pub static CONFIG: OnceCell<Arc<RwLock<Config>>> = OnceCell::const_new();

pub struct Config {
    port: u16,
    methods: HashMap<String, Method>,
}

impl Config {
    pub fn port(&self) -> u16 {
        self.port
    }

    #[allow(dead_code)]
    pub fn methods(&self) -> &HashMap<String, Method> {
        &self.methods
    }

    pub fn method(&self, key: String) -> Option<&Method> {
        self.methods.get(&key)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 3225,
            methods: HashMap::default(),
        }
    }
}

pub async fn init_config() -> Arc<RwLock<Config>> {
    CONFIG
        .get_or_init(|| async {
            Arc::new(RwLock::new(get_config().await))
        })
        .await
        .clone()
}

pub async fn update_config() {
    let config = init_config().await;
    let mut write = config.write().await;
    let new_config = get_config().await;

    write.methods = new_config.methods;
}

async fn get_config() -> Config {
    let config = tokio::fs::read_to_string("/etc/swhook.conf").await;

    match config {
        Ok(string) => {
            let toml = string.parse::<Table>();
            match toml {
                Ok(table) => {
                    let port: u16 = if let Some(t) = table["server"].as_table() {
                        t["port"].as_integer().unwrap_or(3225) as u16
                    } else {
                        3225
                    };

                    let methods: HashMap<String, Method> =
                        if let Some(m) = table["hooks"].as_table().cloned() {
                            m.into_iter()
                                .filter_map(|(k, v)| match v.as_str() {
                                    None => None,
                                    Some(value) => Some((k, Method::from(value.to_string()))),
                                })
                                .collect()
                        } else {
                            HashMap::default()
                        };

                    return Config { port, methods };
                }
                Err(err) => {
                    eprintln!("Failed to load config (toml error) [/etc/swhook.conf]: {err}")
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to load config [/etc/swhook.conf]: {err}")
        }
    }

    println!("Loading standard config...");

    Config::default()
}
