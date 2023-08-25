pub mod config;

use std::env;
use std::error::Error;

pub struct Config {
    cfg: String,
    debug: bool
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // skip the first param
        args.next();

        let cfg = match args.next() {
            Some(arg) => arg,
            None => return Err("need a config file")
        };

        let debug = env::var("debug").is_ok();

        Ok(Config{
            cfg,
            debug
        })
    }
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