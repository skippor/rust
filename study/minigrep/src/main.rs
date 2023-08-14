use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filepath = &args[2];

    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");

    println!("query:{}, filepath:{}, args0:{}", query, filepath, &args[0]);

    println!("Contents: \n{contents}");
}
