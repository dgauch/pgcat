use std::{env, time::Duration};

use redis::{Client, Commands};
use log::info;
use crate::errors::Error;
pub struct MetaConnectionPoolResolver {
    redis_client: Client,
}

static META_LOOKUP_CONNECT_TIMEOUT: u64 = 30;

impl MetaConnectionPoolResolver {
    /// Create a new instance of the resolver.
    pub fn new(meta_lookup_post_port: String, meta_lookup_user: String, meta_lookup_password: String) -> MetaConnectionPoolResolver {
        let host_port = env::var("META_LOOKUP_HOST_PORT").unwrap_or(meta_lookup_post_port.clone());
        let user = env::var("META_LOOKUP_USER").unwrap_or(meta_lookup_user.clone());
        let password = env::var("META_LOOKUP_PASSWORD").unwrap_or(meta_lookup_password.clone());
        let redis_con_str = format!("redis://{}:{}@{}", user, password, host_port);
        
        MetaConnectionPoolResolver {
            redis_client : match redis::Client::open(redis_con_str.clone()) {
                Ok(client) => client,
                Err(e) => panic!("Failed to connect to meta lookup server. Error: {}", e)
            }
        }
    }

    fn get_lookup_connection(&self) -> Result<redis::Connection, Error> {
        match self.redis_client.get_connection_with_timeout(Duration::from_secs(META_LOOKUP_CONNECT_TIMEOUT)) {
            Ok(con) => Ok(con),
            Err(e) => return Err(Error::CannotResolvePool(format!("Failed to connect to meta lookup server. Error: {}", e)))
        }
    }

    pub fn resolve(&self, user_name: &String) -> Result<String, Error> {
        info!("Resolving pool name for user: {}", user_name);
        if user_name.is_empty() {
            return Err(Error::CannotResolvePool("Username is empty".to_string()))
        }
        let lookup_key = format!("r:{}", user_name);
        let lookeup_value = self.get_lookup_connection()?.get(lookup_key).unwrap_or("".to_string());
        if !lookeup_value.is_empty() {
            let conf_cluster_name = lookeup_value.split('=').nth(1).unwrap_or("").to_string();
            if !conf_cluster_name.is_empty() {
                return Ok(conf_cluster_name)
            }
        }
        Err(Error::CannotResolvePool(format!("Cannot resolve pool for username {}", user_name)))
    }
}
