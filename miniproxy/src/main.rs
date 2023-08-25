use std::env;
use std::process;

use miniproxy::Config;

fn main() {
    let only_test = env::var("ONLY_TEST").is_ok();
    if only_test {
        println!("only a test!");
        miniproxy::config::test_clap();
        miniproxy::config::test_ini();

        process::exit(0)
    }


    let cfg = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(-1);
    });

    if let Err(e) = miniproxy::run(cfg) {
        eprintln!("Application error: {e}");
        process::exit(-1);
    }
}
