use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use specs::prelude::*;

use termion::{self, raw::IntoRawMode};

use super::Position;

mod input;

pub use input::InputSys;

pub struct Blademaster;

impl<'a> System<'a> for Blademaster {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        loop {
            print!(
                "{}{}{}",
                termion::clear::All,
                termion::cursor::Hide,
                termion::cursor::Goto(1, 1)
            );
            stdout.flush().unwrap();
            for position in position.join() {
                print!("{}", termion::cursor::Goto(position.x(), position.y()));
                println!("$");
            }
            stdout.flush().unwrap();

            thread::sleep(Duration::from_millis(100));
        }
    }
}
