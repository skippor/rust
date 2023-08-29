pub mod config;
pub mod tunnel;
pub mod route;
pub mod crypto;


use std::env;
use std::error::Error;

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