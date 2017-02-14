use frontend::event::Event;
use frontend::event::Event::*;
use std::io::{self, Write, stdout, Stdout};
use termion::terminal_size;

pub struct Terminal {
    stdout: Stdout,
    width: u16,
    height: u16,
}

impl Terminal {
    pub fn new() -> Self {
        let stdout = stdout();
        // let mut stdout = stdout.lock();
        let terminal_size = terminal_size().ok();

        Terminal {
            stdout: stdout,
            width: terminal_size.map(|(w, _)| w - 2).unwrap_or(80),
            height: terminal_size.map(|(_, h)| h - 2).unwrap_or(40),
        }
    }

    pub fn update(&mut self, event: Event) {
        match event {
            MenuInit => {
                println!("{:?}", 1);
            },
            _ => panic!("Unfinished")
        }
    }
}
