use std::io::{self, Write};
use std::time::Duration;

use crossterm::cursor::{self, Show, MoveTo};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, EnableLineWrap};
use crossterm::style::{PrintStyledContent, Print, Attribute, Color, Stylize};
use crossterm::{QueueableCommand, execute, event};

mod game;
mod snake;
mod clock;
use game::Game;
use game::GraphicsObject;

fn main() -> io::Result<()> {

    let (w, h) = terminal::size()?;
    let mut framebuffer = Render::new(w, h);
    framebuffer.init_screen()?;

    let mut game = Game::new(w, h, framebuffer);
    game.start();
    loop {
        game.run();
        if event::poll(Duration::from_secs(0))? && let Event::Key(event) = event::read()? {
            match (event.code, event.modifiers) {
                (KeyCode::Char('q'), _) => break,
                (KeyCode::Char('c'), KeyModifiers::CONTROL) => break,
                (c, _) => game.process_keystroke(c),
            }
        }
        if game.game_over {
            break;
        }
    }
    game.end();
     Ok(())
}



struct Render {
    w: u16,
    h: u16,
    stdout: io::Stdout
}

impl Render {
    fn new(w: u16, h: u16) -> Render{
        Render {w, h, stdout: io::stdout()}
    }

    fn init_screen(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, cursor::Hide)?;
        self.stdout
            .queue(EnableLineWrap)?
            .queue(MoveTo(0, 0))?;

        // first line
        self.stdout.queue(Print('+'))?;
        for _ in 1..self.w-1 {
            self.stdout.queue(Print('-'))?;
        }
        self.stdout.queue(Print('+'))?;
        // left and right border
        for _ in 1..self.h-1 {
            self.stdout.queue(Print('|'))?;
            for _ in 1..(self.w-1) {
                self.stdout.queue(Print(' '))?;
            }
            self.stdout.queue(Print('|'))?;
        }
        // last line
        self.stdout.queue(Print('+'))?;
        for _ in 1..self.w-1 {
            self.stdout.queue(Print('-'))?;
        }
        self.stdout.queue(Print('+'))?;

        self.stdout.flush()
    }
    fn draw_at(&mut self, glyph: GraphicsObject, x: u16, y: u16) -> io::Result<()> {
        self.stdout
            .queue(MoveTo(x, y))?
            .queue(PrintStyledContent(match glyph {
                GraphicsObject::SnakeSegment => "O".with(Color::Grey),
                GraphicsObject::Food => "*".with(random_color()).attribute(Attribute::Bold),
                GraphicsObject::Empty => " ".with(Color::Grey),
            }))?;
        Ok(())
    }
    fn flip(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
    fn write_header(&mut self, text: String, _len: u16) -> io::Result<()> {
            self.stdout.queue(MoveTo(3, 0))?.queue(Print(text))?;
            self.stdout.flush()
    }

    fn cleanup_screen(&mut self) {
        execute!(io::stdout(), LeaveAlternateScreen, Show).ok();
        terminal::disable_raw_mode().ok();
    }
}
use rand::prelude::IndexedRandom;


fn random_color() -> Color {
    *[Color::Blue, Color::Yellow, Color::Magenta,
        Color::Cyan, Color::Red, Color::Green].choose(&mut rand::rng()).expect("Gówno")
}