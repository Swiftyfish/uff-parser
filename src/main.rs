use std::fs::File;
use std::vec::Vec;

struct UFF_File {
   frequency_interval : f64,
   sample_count : usize,
   data : Vec<T>,
}

fn main() -> std::io::Result<()> {
    let mut file = File::open("uff_file.uff")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(())
}

