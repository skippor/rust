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
        let matches = App::new("MayApp")
            .version("0.1")
            .author("kayryu")
            .about("Learn use Rust Crate!")
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

pub fn test_ini() {
    let i = Ini::load_from_file("conf/miniproxy.ini").unwrap();
    for (sec, prop) in i.iter() {
        println!("Section: {:?}", sec);
        for (k, v) in prop.iter() {
            println!("{}:{}", k, v);
        }
    }
}

pub fn test_clap() {
    let matches = App::new("MayApp")
        .version("0.1")
        .author("kayryu")
        .about("Learn use Rust Crate!")
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
    }

    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("list") {
            println!("Printing testing lists...");
        } else {
            println!("Not printing testing lists...");
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
