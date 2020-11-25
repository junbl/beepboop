use std::env;
use std::process;

fn main() {
    for file in env::args() {
        println!("parsing file {:?}",file);

    }
    // println!("{:?}",args);
}
