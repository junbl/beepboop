use std::env;

mod fileparser;
mod interpreter;
mod types;

fn main() {
    let mut arguments = env::args();
    arguments.next();
    for file in arguments {
        println!("parsing file {:?}", file);

        let bbfile = fileparser::BeepboopFile::new(file);

        let program = bbfile.parse_file();
        let final_state = program.run_program();

        println!("{:?}", final_state);
    }
}
