pub mod config;
pub mod tunnel;
pub mod route;
pub mod crypto;


use std::env;
use std::error::Error;

pub struct Config {
    cfg: String,
    debug: bool
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("run config is {}", config.cfg);
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