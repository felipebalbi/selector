use indoc::indoc;
use rand::Rng;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Position, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Padding, Paragraph, StatefulWidget, Widget, Wrap},
};
use tui_big_text::{BigText, PixelSize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MerryXmas;

impl MerryXmas {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn as_str(&self) -> &'static str {
        indoc! {"
                 ▄▄▄  ▄▄▄                                                    ▄▄▄  ▄▄▄                               
                 ███  ███                                                     ██▄▄██                                
                 ████████   ▄████▄    ██▄████   ██▄████  ▀██  ███              ████    ████▄██▄   ▄█████▄  ▄▄█████▄ 
                 ██ ██ ██  ██▄▄▄▄██   ██▀       ██▀       ██▄ ██                ██     ██ ██ ██   ▀ ▄▄▄██  ██▄▄▄▄ ▀ 
                 ██ ▀▀ ██  ██▀▀▀▀▀▀   ██        ██         ████▀               ████    ██ ██ ██  ▄██▀▀▀██   ▀▀▀▀██▄ 
                 ██    ██  ▀██▄▄▄▄█   ██        ██          ███               ██  ██   ██ ██ ██  ██▄▄▄███  █▄▄▄▄▄██ 
                 ▀▀    ▀▀    ▀▀▀▀▀    ▀▀        ▀▀          ██               ▀▀▀  ▀▀▀  ▀▀ ▀▀ ▀▀   ▀▀▀▀ ▀▀   ▀▀▀▀▀▀  
                                                          ███                                                       
        "}
    }
}

impl Widget for MerryXmas {
    fn render(self, area: Rect, buf: &mut Buffer) {
        BigText::builder()
            .centered()
            .pixel_size(PixelSize::Full)
            .lines(vec!["Merry Xmas!".red().into(), "~~~~~~~~~~~".into()])
            .build()
            .render(area, buf);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Help;

impl Help {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Widget for Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Press 'q' to quit the application or 'Space' to reveal the next pair")
            .block(Block::bordered().title("Help").padding(Padding::uniform(1)))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}

pub struct PairsState {
    started: bool,
    data: Box<dyn Iterator<Item = (String, String)>>,
    from: String,
    to: String,
}

impl PairsState {
    #[must_use]
    pub fn new(data: Vec<(String, String)>) -> Self {
        Self {
            started: false,
            data: Box::new(data.into_iter()),
            from: String::from("Press <SPC>"),
            to: String::from("start"),
        }
    }

    pub fn start(&mut self) {
        self.started = true;
    }

    pub fn tick(&mut self) {
        if let Some((from, to)) = self.data.next() {
            self.from = from;
            self.to = to;
        } else {
            self.from = "Press <Q>".into();
            self.to = "exit".into();
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Pairs;

impl Pairs {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl StatefulWidget for Pairs {
    type State = PairsState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let padding = 3;

        // big text
        let text = BigText::builder()
            .centered()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![
                state.from.clone().blue().into(),
                "to".into(),
                state.to.clone().blue().into(),
            ])
            .build();

        // compute the height of the big text
        let para_height = 11 + padding * 2;

        // center inside the area
        let centered_rect = Rect {
            x: area.x,
            y: area.y + (area.height.saturating_sub(para_height)) / 2,
            width: area.width,
            height: para_height.min(area.height),
        };

        // render
        text.render(centered_rect, buf);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct SnowField {
    width: u16,
    height: u16,
    count: usize,
    flakes: Vec<(u16, u16)>,
}

impl SnowField {
    #[must_use]
    pub fn new(count: usize) -> Self {
        Self {
            width: 0,
            height: 0,
            count,
            flakes: vec![],
        }
    }

    pub fn set_size(&mut self, width: u16, height: u16) {
        let mut rng = rand::rng();
        let flakes = (0..self.count)
            .map(|_| (rng.random_range(0..width), rng.random_range(0..height)))
            .collect();

        self.width = width;
        self.height = height;
        self.flakes = flakes;
    }

    pub fn tick(&mut self) {
        // move flakes down by 1, wrap around
        for flake in &mut self.flakes {
            flake.1 = (flake.1 + 1) % self.height;
        }

        // randomly add/remove a few flakes for twinkle
        let mut rng = rand::rng();
        if rng.random_bool(0.3) {
            let i = rng.random_range(0..self.flakes.len());
            self.flakes[i] = (
                rng.random_range(0..self.width),
                rng.random_range(0..self.height),
            );
        }
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        for &(x, y) in &self.flakes {
            if x < area.width && y < area.height {
                buf[Position {
                    x: area.x + x,
                    y: area.y + y,
                }]
                .set_symbol("*")
                .set_style(Style::default().fg(Color::White));
            }
        }
    }
}
