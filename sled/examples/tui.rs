use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Circle, Context},
        *,
    },
};

use sled::{color::Srgb, Sled, Vec2};

use std::{
    io::{self, stdout, Error, ErrorKind, Stdout},
    ops::Range,
};

pub struct SledTerminalDisplay {
    title: String,
    pub leds: Vec<(Srgb<u8>, Vec2)>,
    on_quit: Box<dyn FnMut()>,
    quit: bool,
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

#[allow(dead_code)]
impl SledTerminalDisplay {
    pub fn start(title: &str, domain: Range<Vec2>) -> Self {
        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();
        SledTerminalDisplay {
            title: String::from(title),
            leds: vec![],
            x_bounds: [domain.start.x as f64, domain.end.x as f64],
            y_bounds: [domain.start.y as f64, domain.end.y as f64],
            terminal: Terminal::new(CrosstermBackend::new(stdout())).unwrap(),
            on_quit: Box::new(|| {}),
            quit: false,
        }
    }

    pub fn on_quit(&mut self, callback: impl FnMut() + 'static) {
        self.on_quit = Box::new(callback);
    }

    pub fn stop(&mut self) -> io::Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        let should_quit = self.check_for_quit()?;
        if should_quit {
            self.quit = true;
            (self.on_quit)();
            self.stop()?;
            Err(Error::new(ErrorKind::Other, "User closed the terminal."))
        } else {
            self.draw_frame()?;
            Ok(())
        }
    }

    fn check_for_quit(&self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_nanos(1))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    fn draw_frame(&mut self) -> io::Result<()> {
        let canvas = build_viewport(&self.title, self.x_bounds, self.y_bounds).paint(|ctx| {
            for led in &self.leds {
                draw_led(ctx, led);
            }
        });

        self.terminal.draw(|frame| {
            frame.render_widget(canvas, frame.size());
        })?;

        Ok(())
    }
}

impl Drop for SledTerminalDisplay {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}

fn draw_led(ctx: &mut Context, led: &(Srgb<u8>, Vec2)) {
    let (col, pos) = led;
    ctx.draw(&Circle {
        x: pos.x as f64,
        y: pos.y as f64,
        radius: 0.0,
        color: Color::Rgb(col.red, col.green, col.blue),
    });
}

fn build_viewport<T>(title: &str, x_bounds: [f64; 2], y_bounds: [f64; 2]) -> Canvas<'_, T>
where
    T: Fn(&mut Context<'_>),
{
    Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(title))
        .marker(Marker::HalfBlock)
        .background_color(Color::Black)
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
}

#[allow(dead_code)]
fn main() -> io::Result<()> {
    let sled = Sled::new("./examples/config.toml").unwrap();

    let mut display = SledTerminalDisplay::start("Sled Visualizer", sled.domain());
    display.leds = sled.read_colors_and_positions();
    display.refresh()?;

    Ok(())
}
