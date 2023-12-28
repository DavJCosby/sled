use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Circle, Rectangle},
        *,
    },
};

use sled::{color::Srgb, Sled, Vec2};

use std::io::{self, stdout, Stdout};

pub struct SledTerminalDisplay {
    pub leds: Vec<(Srgb<u8>, Vec2)>,
    on_quit: Box<dyn FnMut()>,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    terminal: Option<Terminal<CrosstermBackend<Stdout>>>,
}

impl SledTerminalDisplay {
    pub fn new() -> Self {
        SledTerminalDisplay {
            leds: vec![],
            x_bounds: [-2.5, 3.75],
            y_bounds: [-1.1, 2.1],
            terminal: None,
            on_quit: Box::new(|| {}),
        }
    }

    pub fn start(&mut self) -> io::Result<()> {
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        self.terminal = Some(Terminal::new(CrosstermBackend::new(stdout()))?);
        Ok(())
    }

    pub fn on_quit(&mut self, callback: impl FnMut() + 'static) {
        self.on_quit = Box::new(callback);
    }

    pub fn stop(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        let should_quit = self.check_for_quit()?;
        if !should_quit {
            self.draw()?;
        } else {
            (self.on_quit)();
            self.stop()?;
        }
        Ok(())
    }

    fn draw(&mut self) -> io::Result<()> {
        if let Some(terminal) = &mut self.terminal {
            terminal.draw(|frame| {
                frame.render_widget(
                    canvas::Canvas::default()
                        .block(Block::default().borders(Borders::ALL).title("Current effect: quirky_trail.rs"))
                        .marker(Marker::HalfBlock)
                        .background_color(Color::Black)
                        .paint(|ctx| {
                            for (col, pos) in &self.leds {
                                ctx.draw(&Circle {
                                    x: pos.x as f64,
                                    y: pos.y as f64,
                                    radius: 0.0,
                                    color: Color::Rgb(col.red, col.green, col.blue),
                                });
                            }
                        })
                        .x_bounds(self.x_bounds)
                        .y_bounds(self.y_bounds),
                    frame.size(),
                );
            })?;
        }
        Ok(())
    }

    fn check_for_quit(&self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
}

fn ui(frame: &mut Frame, canvas: impl Widget) {
    frame.render_widget(canvas, frame.size())
}

fn main() -> io::Result<()> {
    let mut display = SledTerminalDisplay::new();
    let sled = Sled::new("./examples/config.toml").unwrap();
    let colors = sled.read_colors::<u8>();
    let positions = sled.read_positions();
    let led_data = colors.into_iter().zip(positions.into_iter()).collect();
    display.leds = led_data;
    display.start()?;
    display.refresh()?;
    //display.stop()?;
    Ok(())
}
