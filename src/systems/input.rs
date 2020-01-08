use std::{
    io::{stdin, stdout, Write},
    process,
};

use specs::prelude::*;

use termion::{self, event::Key, input::TermRead, raw::IntoRawMode};

use super::Position;

pub struct InputSys;

impl<'a> System<'a> for InputSys {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, _: Self::SystemData) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        loop {
            let stdin = stdin();

            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char('q') => {
                        print!("{}", termion::cursor::Restore);
                        stdout.flush().unwrap();
                        process::exit(1);
                    }
                    _ => (),
                }
            }
            stdout.flush().unwrap();
        }
    }
}
