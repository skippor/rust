use std::env;

extern crate clap;
use ini::Ini;
use clap::{Arg, App, SubCommand, load_yaml};

pub struct ProxyConfig {
    addr: String,
    proto: String,
    debug: bool
}

impl ProxyConfig {
    pub fn load_from_file(&self, filename: &str) -> ProxyConfig {
        let ini = Ini::load_from_file(filename).unwrap();
        let addr = ini.get_from_or(Some("server"), "addr", "127.0.0.1").to_string();
        let proto = ini.get_from_or(Some("server"), "proto", "TCP").to_string();
        let debug = env::var("debug").is_ok();
    
        ProxyConfig {
            addr,
            proto,
            debug
        }
    }

    pub fn build_from_args(&self) -> ProxyConfig {
        let matches = App::new("miniproxy")
            .version("1.0.0")
            .author("skippor")
            .about("A mini proxy ")
            .arg(Arg::with_name("verbose")
                .short("v")
                .multiple(true)
                .help("verbosity level"))
            .args_from_usage("-p, --path=[FILE] 'Target file you want to change'")
            .subcommand(SubCommand::with_name("test")
                            .about("does testing things")
                            .arg_from_usage("-l, --list 'lists test values'"))
            .get_matches();

        if let Some(f) = matches.value_of("path") {
            println!("path : {}", f);
            return self.load_from_file(f);
        }

        if let Some(matches) = matches.subcommand_matches("test") {
            if matches.is_present("list") {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }

        ProxyConfig {
            addr: "localhost".to_string(),
            proto: "UDP".to_string(),
            debug: false
        }
    }
}

pub fn test_ini(filename: &str) {
    let i = Ini::load_from_file(filename).unwrap();
    for (sec, prop) in i.iter() {
        println!("Section: {:?}", sec);
        for (k, v) in prop.iter() {
            println!("{}:{}", k, v);
        }
    }
}

pub fn test_clap() {
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
                            .arg_from_usage("-s, --service=[SERVICE] 'service to be proxyed by the server'")
                        )
            .get_matches();

    if let Some(f) = matches.value_of("filepath") {
        return test_ini(f);
    }

    if let Some(matches) = matches.subcommand_matches("server") {
        if let Some(addr) = matches.value_of("listen") {
            println!("Run as Server, listening on {addr} ...");
        }
    }

    if let Some(matches) = matches.subcommand_matches("client") {
        if let Some(addr) = matches.value_of("connect") {
            println!("Run as client, connecting on {addr} ...");
        }

        if let Some(service) = matches.value_of("service") {
            println!("Run as client, service open with {service} ...");
        }
    }
}

pub fn test_args() {
    let yml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yml).get_matches();

    let _ = match matches.occurrences_of("verbose") {
        0 => println!("zero"),
        1 => println!("one"),
        _ => println!("more")
    };

    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("list") {
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
        }
    }
}
