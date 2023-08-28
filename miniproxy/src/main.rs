use std::process;


fn main() {
    miniproxy::config::test_clap();
    miniproxy::config::test_ini();

    process::exit(0)
}
