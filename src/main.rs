use clap::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::{error::Error, process};

/// Searches for tool data in the given csv-file and saves it to tools.yml.
/// > read_csv <filename> -> tools.yml
#[derive(Parser)]
struct Cli {
    /// The path and filename to read.
    path: std::path::PathBuf,
}

fn run(args: Cli) -> Result<(), Box<dyn Error>> {
    //file open
    let file = File::open(&args.path).expect("open input file");
    //read csv
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    //define HashMap
    let mut tools: HashMap<String, String> = HashMap::new();
    //fill HashMap
    for result in rdr.records() {
        let record = result?;
        tools.insert(record[0].to_string(), record[2].to_string());
        //println!("{:?}", record);
    }
    //transfer it to yaml
    let yaml = serde_yaml::to_string(&tools).expect("to yaml string");
    // write to file
    std::fs::write("tools.yml", &yaml)?;

    Ok(())
}

fn main() {
    let args = Cli::parse();

    if let Err(err) = run(args) {
        println!("Error running read_csv: {}", err);
        process::exit(1);
    }
}
