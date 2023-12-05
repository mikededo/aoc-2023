// use std::{fs::File, process};
use std::fs::read_to_string;

// pub fn read(path: &str) -> File {
//     let file = File::open(path);
//     match file {
//         Ok(f) => f,
//         Err(e) => {
//             eprintln!("Error: {e}");
//             process::exit(1);
//         }
//     }
// }

pub fn read_lines(path: &str) -> Vec<String> {
    read_to_string(format!("./src/data/{path}"))
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
