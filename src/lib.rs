use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
};
use termion::{color, style};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(
    author = "Felipe Balbi <felipe@balbi.sh>",
    about = "Random selection of lines from FILE"
)]
pub struct Config {
    #[arg(
        num_args = 1,
        required = true,
        value_name = "FILE",
        help = "Input file"
    )]
    file: String,
}

struct Draw<'a> {
    from: &'a str,
    to: &'a str,
}

impl<'a> Draw<'a> {
    fn new(from: &'a str, to: &'a str) -> Self {
        Self { from, to }
    }
}

impl<'a> Display for Draw<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{} {}-> {}{}{}",
            style::Bold,
            color::Fg(color::Red),
            self.from,
            color::Fg(color::Yellow),
            color::Fg(color::Green),
            self.to,
            style::Reset
        )
    }
}

pub fn run(config: Config) -> Result<()> {
    match File::open(&config.file) {
        Err(e) => eprintln!("Failed to open {}: {}", config.file, e),
        Ok(file) => {
            let reader = BufReader::new(file);
            let lines = reader
                .lines()
                .map(|l| match l {
                    Ok(line) => line,
                    Err(e) => {
                        eprintln!("Failed to read line: {}", e);
                        String::new()
                    }
                })
                .collect::<Vec<_>>();
            let mut rng = thread_rng();
            let mut data = cartesian_product(&lines)?;

            data.shuffle(&mut rng);
            println!("Press <RET> to continue");

            for draw in data {
                let stdin = io::stdin();
                let mut handle = stdin.lock();
                let mut buffer = String::new();

                handle.read_line(&mut buffer)?;

                println!("{}", draw);
            }
        }
    }

    Ok(())
}

fn cartesian_product<'a>(input: &'a [String]) -> Result<Vec<Draw>> {
    let pairs = input
        .iter()
        .flat_map(|fst| {
            input
                .iter()
                .map(|snd| Draw::new(fst, snd))
                .collect::<Vec<_>>()
        })
        .filter(|draw| draw.from != draw.to)
        .collect::<Vec<_>>();

    Ok(pairs)
}
