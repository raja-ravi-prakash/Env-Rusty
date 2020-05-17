mod vendor;
use colored::Colorize;
use std::env;

fn main() {
    let name = env::args().nth(1);
    let key = env::args().nth(2);
    let op = env::args().nth(3);
    let option = op.as_deref().unwrap();

    if option == "--env" {
        vendor::set(name, key);
    } else if option == "--reg" {
        let path = env::args().nth(4);
        vendor::setr(name, key, path);
    } else {
        print!("{}", "error : ".red());
        print!("No option {} found\n", option);
    }
}
