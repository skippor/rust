use std::process;

fn main() {
    let proxy_config = miniproxy::config::build().unwrap_or_else(|err| {
        eprintln!("Problem loading proxy config: {err}");
        process::exit(1);
    });

    println!("config:{:?}", proxy_config);

    if let Err(e) = miniproxy::run(proxy_config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
