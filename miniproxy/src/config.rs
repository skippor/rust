use std::env;

extern crate clap;
use ini::Ini;
use clap::{Arg, App, SubCommand};

#[derive(Debug)]
pub struct ProxyConfig {
    //name: String,
    //token: String,
    addr: String,
    proto: String,
    //server: bool,
    debug: bool
}

impl ProxyConfig {
    pub fn load_from_file(filename: &str) -> Result<ProxyConfig, &'static str> {
        let ini = Ini::load_from_file(filename).unwrap();

        for (sec, prop) in ini.iter() {
            println!("Section: {:?}", sec);
            for (k, v) in prop.iter() {
                println!("{}:{}", k, v);
            }
        }

        let addr = ini.get_from_or(Some("server"), "addr", "127.0.0.1").to_string();
        let proto = ini.get_from_or(Some("server"), "proto", "TCP").to_string();
        let debug = env::var("debug").is_ok();
    
        Ok(ProxyConfig {
            addr,
            proto,
            debug
        })
    }
}

pub fn build() -> Result<ProxyConfig, &'static str> {
    let matches = App::new("miniproxy")
        .version("1.0.0")
        .author("skippor")
        .about("A mini proxy for study!")
        .arg(Arg::with_name("verbose")
            .short("v")
            .multiple(true)
            .help("verbosity level"))
        .args_from_usage("-f, --filepath=[FILE] 'config file you want to use'")
        .subcommand(SubCommand::with_name("server")
                        .about("run as a server")
                        .arg_from_usage("-l, --listen=[ADDRESS] 'server address to listen'")
                    )
        .subcommand(SubCommand::with_name("client")
                        .about("run as a client")
                        .arg_from_usage("-c, --connect=[ADDRESS] 'server address to connect'")
                        .arg_from_usage("-m, --map=[lport:rport] 'service port to be proxyed by the server'")
                    )
        .get_matches();

    if let Some(f) = matches.value_of("filepath") {
        return ProxyConfig::load_from_file(f);
    }

    if let Some(matches) = matches.subcommand_matches("server") {
        if let Some(addr) = matches.value_of("listen") {
            println!("Run as Server, listening on {addr} ...");
        }

        return Ok(ProxyConfig {
            addr: "server".to_string(),
            proto: "UDP".to_string(),
            debug: false
        });
    }

    if let Some(matches) = matches.subcommand_matches("client") {
        if let Some(addr) = matches.value_of("connect") {
            println!("Run as client, connecting on {addr} ...");
        }

        if let Some(service) = matches.value_of("map") {
            println!("Run as client, service open with {service} ...");
        }

        return Ok(ProxyConfig {
            addr: "client".to_string(),
            proto: "UDP".to_string(),
            debug: false
        });
    }

    Ok(ProxyConfig {
        addr: "localhost".to_string(),
        proto: "UDP".to_string(),
        debug: false
    })
}