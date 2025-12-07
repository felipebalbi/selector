use clap::Parser;
use color_eyre::{eyre::eyre, Result};
use itertools::{iproduct, Itertools};
use rand::rng;
use rand::seq::SliceRandom;
use ratatui::{
    buffer::Buffer,
    layout::{
        Constraint::{Fill, Length},
        Layout, Rect,
    },
    widgets::{StatefulWidget, Widget},
    Frame,
};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::widgets::{Help, MerryXmas, Pairs, PairsState};

pub mod widgets;

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

pub struct App {
    state: PairsState,
}

impl App {
    pub fn new(config: Config) -> Self {
        let data = generate_pairs(&config.file).unwrap_or(vec![]);
        let state = PairsState::new(data);

        Self { state }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    pub fn handle_spacebar(&mut self) {
        self.state.start();
        self.state.tick();
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [top, mid, bottom] = Layout::vertical([Length(12), Fill(1), Length(5)]).areas(area);

        MerryXmas::new().render(top, buf);
        Pairs::new().render(mid, buf, &mut self.state);
        Help::new().render(bottom, buf);
    }
}

pub fn generate_pairs(path: &str) -> Result<Vec<(String, String)>> {
    match File::open(&path) {
        Err(e) => Err(eyre!("Failed to open '{}': {}", path, e)),
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
            Ok(data)
        }
    }
}

fn cartesian_product(lines: Vec<String>) -> Vec<(String, String)> {
    iproduct!(lines.clone().into_iter(), lines.into_iter())
        .filter(|(a, b)| a != b)
        .collect_vec()
}
