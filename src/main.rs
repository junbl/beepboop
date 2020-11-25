use std::env;

mod fileparser;
mod interpreter;
mod types;

fn main() {
    for file in env::args() {
        println!("parsing file {:?}",file);
        let bbfile = fileparser::BeepboopFile::new(file);

        let program = bbfile.parse_file();
        let final_state = program.run_program();
        println!("{:?}",final_state);

    }
    // println!("{:?}",args);
}
