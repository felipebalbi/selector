use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(
    author = "Felipe Balbi <felipe@balbi.sh>",
    about = "Random selection of lines from FILE"
)]
pub struct Config {
    #[arg(
        num_args = 1,
        default_value = "-",
        value_name = "FILE",
        help = "Input file"
    )]
    file: String,
}

pub fn run(config: Config) -> Result<()> {
    match open(&config.file) {
        Err(e) => eprintln!("Failed to open {}: {}", config.file, e),
        Ok(reader) => {
            let mut lines = reader.lines().collect::<Vec<_>>();
            let mut rng = thread_rng();

            lines.shuffle(&mut rng);
            println!("Press <RET> to continue");

            for line in lines {
                let line = line?;
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                let mut buffer = String::new();

                handle.read_line(&mut buffer)?;

                println!("{line}");
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
