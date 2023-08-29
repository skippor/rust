pub mod config;
pub mod tunnel;
pub mod route;
pub mod crypto;

use std::error::Error;
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    static ref g_config: HashMap<&'static str, Box<config::ProxyConfig>> = {
        let mut m = HashMap::new();
        m
    };
}

pub fn run(config: config::ProxyConfig) -> Result<(), Box<dyn Error>> {
    println!("run config is {:?}", config);
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_eq() {
        assert_eq!(2, 1 + 1);
    }
}