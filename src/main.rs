use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec::Vec;

enum UFF_Type {
    V1601 {
        frequency_spacing : f64,
        amplitude_data : [f64; 1601],
    },
    V3201 {
        frequency_spacing : f64,
        amplitude_data : [f64; 3201],
    },
}
    
fn main() -> std::io::Result<()> {
    let file = File::open("uff_file.uff")?;

    let contents = String::new();

    file.read_to_string(&mut contents)?;

    let lines : Vec<&str> = contents.split(" -1")
        .filter(|x| !x.is_empty())
        .collect();


   // let mut output : Vec<UFF_Type> = Vec::new()

    Ok(())
}

