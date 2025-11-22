//! # [Ratatui] Sparkline example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui/ratatui
//! [examples]: https://github.com/ratatui/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui/ratatui/blob/main/examples/README.md

use std::time::{Duration, Instant};

use color_eyre::Result;
use rand::{
    distr::{Distribution, Uniform},
    rngs::ThreadRng,
};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Sparkline},
    DefaultTerminal, Frame,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

struct App {
    signal: RandomSignal,
    pings: Vec<u64>,
}

#[derive(Clone)]
struct RandomSignal {
    distribution: Uniform<u64>,
    rng: ThreadRng,
}

impl RandomSignal {
    fn new(lower: u64, upper: u64) -> Self {
        Self {
            distribution: Uniform::new(lower, upper).unwrap(),
            rng: rand::rng(),
        }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(self.distribution.sample(&mut self.rng))
    }
}

impl App {
    fn new() -> Self {
        let mut signal = RandomSignal::new(0, 100);
        let pings = signal.by_ref().take(200).collect::<Vec<u64>>();
        Self {
            signal,
            pings,
        }
    }

    fn on_tick(&mut self) {
        let target_ip = "8.8.8.8".parse().unwrap();
        let data = [1,2,3,4];  // ping data
        let timeout = Duration::from_secs(1);
        let options = ping_rs::PingOptions { ttl: 117, dont_fragment: true };
        let result = ping_rs::send_ping(&target_ip, timeout, &data, Some(&options));

        let ping_time =match result {
            Ok(reply) => reply.rtt as u64,
            Err(e) => 100, 
        };
        println!("{}",ping_time);

        self.pings.pop();
        self.pings.insert(0, ping_time);
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(1000);

        let mut last_tick = Instant::now();
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

        let sparkline = Sparkline::default()
            .block(
                Block::new()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .title("Data3"),
            )
            .data(&self.pings)
            .style(Style::default().fg(Color::Red));
        //frame.render_widget(sparkline, chunks[2]);
    }
}
