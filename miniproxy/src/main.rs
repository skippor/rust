use std::process;


fn main() {
    let proxy_config = miniproxy::config::build_config().unwrap_or_else(|err| {
        eprintln!("Problem loading proxy config: {err}");
        process::exit(1);
    });

    println!("config:{:?}", proxy_config);
}
