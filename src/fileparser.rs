use std::fs;

pub mod fileparser {

    pub struct BeepboopFile {
        pub filename: String,
        pub lines: Vec<&str>,
    }

    impl BeepboopFile {
        pub fn new(new_filename: &String) -> Result<BeepboopFile, &'static str> {
            let filename = new_filename.clone();
            let contents = fs::read_to_string(filename)?;
            let lines = contents.collect::<Vec<&str>>();
        }
        fn parse_num(num_iter: Iterator::&str) -> Result<i32,Box<dyn Error>> {
            let mut bin_str = "";
            let mut digit_list = Vec<bool>::new();
            while let Some(digit) = num_iter.next() {
                match digit {
                    "beep" => 1,
                    "boop" => 0,
                    other => eprintln!("Failed to parse at {}",other),
                }
            }
        }

        pub fn parse_cmd(state: ProgramState, cmd: &str) -> Result<Expr, Box<dyn Error>> {
            let mut cmd_iter = cmd.split_whitespace();
            match cmd_iter.next() {
                Some("whirr") => {
                    let var_name = cmd_iter.next();
                    let value = cmd_iter.next() match {
                        Some("beep") | Some("boop") => Const(parse_num(cmd_iter)),
                    }
                    Assign(var_name,value)
                },
                None => {},
                other => eprintln!("Failed to parse at {}",other),
            }
        }

        // pub fn run_program() -> Result<ProgramState, Box<dyn Error>> {
        //     let initial_state = ProgramState::new();
        //     lines.iter().fold(initial_state, run_cmd);
        // }
    }
}
