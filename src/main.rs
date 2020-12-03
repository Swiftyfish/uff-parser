use std::env;
use std::convert::TryInto;
use std::fs;
use std::io;
use std::vec::Vec;

enum UffType<'a> {
    V1601 {
        date : &'a str,
        frequency_spacing : f64,
        amplitude_data : Vec<f64>,
    },
    V3201 {
        date: &'a str,
        frequency_spacing : f64,
        amplitude_data : Vec<f64>, 
    },
}
    
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = parse_config(&args);

    let contents = fs::read_to_string(filename)?;

    let samples : Vec<&str> = contents.split(" -1")
        .filter(|x| !x.trim().is_empty())
        .collect();

    println!("{}", samples.len());
    
    let uff_data = parse_sample(samples.get(0).unwrap())?;

    println!("{}", uff_data.amplitude_data.len());
    
    Ok(())
}

fn parse_config(args: &[String]) -> &str {
    if args.len() < 2 {
        panic!("Please provide filename");
    }

    let filename = &args[1];

    filename
}

fn parse_sample(sample: &str) -> Result<UffType, Box<dyn Error> {
    // Split raw string into lines, stopping after 13 to allow for separation
    // of amplitude data in a single line.

    let lines : Vec<&str> = sample.splitn(13, "\n")
        .collect();

    //Extract data and frequency spacing information from corresponding lines.

    let date = &lines[4];

    let freq_interval = &lines[8]
        .split_whitespace()
        .nth(4)
        .unwrap()
        .parse::<f64>()?;

    let amp_data : Vec<f64> = lines.last()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", amp_data.len());

    match amp_data.len() {
        1601 => Ok(UffType::V1601 {
            date : *date,
            frequency_spacing : *freq_interval,
            amplitude_data : amp_data
        }),
        3201 => Ok(UffType::V3201 {
            date : *date,
            frequency_spacing : *freq_interval,
            amplitude_data : amp_data
        }),
        _ => Err("Unknown UFF file length"),
    }
}
