use clap::Parser;
use itertools::{iproduct, Itertools};
use rand::rng;
use rand::seq::SliceRandom;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{stdin, stdout, BufRead, BufReader, Write},
};
use termion::{color, event::Key, input::TermRead, raw::IntoRawMode, style, terminal_size};

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

struct Draw {
    from: String,
    to: String,
}

impl Draw {
    fn new(from: String, to: String) -> Self {
        Self { from, to }
    }
}

impl Display for Draw {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{:>10} {}{} {}{}{:<10}",
            style::Bold,
            color::Fg(color::Red),
            self.from,
            color::Fg(color::Yellow),
            "→",
            color::Fg(color::Green),
            self.to,
            style::Reset
        )
    }
}

pub fn run(config: Config) -> Result<()> {
    let (width, height) = terminal_size()?;
    let center_col = 1 + width / 2;
    let center_row = 1 + height / 2;

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
            let mut rng = rng();
            let mut data = cartesian_product(lines);

            data.shuffle(&mut rng);

            let stdin = stdin();
            let mut stdout = stdout().into_raw_mode()?;

            write!(
                stdout,
                "{}{}Press any key to continue...{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1),
                termion::cursor::Hide
            )?;

            write!(
                stdout,
                "{}🎅🎄🤶",
                termion::cursor::Goto(center_col - 1, center_row + 1)
            )?;

            stdout.flush()?;

            let mut data = data.iter();

            for key in stdin.keys() {
                let key = key?;

                match key {
                    Key::Ctrl('q') | Key::Ctrl('c') => {
                        break;
                    }
                    _ => {
                        if let Some(draw) = data.next() {
                            write!(
                                stdout,
                                "{}{}{}",
                                termion::cursor::Goto(center_col - 10, center_row),
                                termion::clear::CurrentLine,
                                draw
                            )?;
                            stdout.flush()?;
                        } else {
                            break;
                        }
                    }
                }
            }

            write!(
                stdout,
                "{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1)
            )?;
            stdout.flush()?;
            write!(stdout, "{}", termion::cursor::Show).unwrap();
        }
    }

    Ok(())
}

fn cartesian_product(lines: Vec<String>) -> Vec<Draw> {
    iproduct!(lines.clone().into_iter(), lines.into_iter())
        .filter(|(a, b)| a != b)
        .map(|(a, b)| Draw::new(a, b))
        .collect_vec()
}
