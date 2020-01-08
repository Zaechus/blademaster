use std::{thread, time::Duration};

use specs::prelude::*;

use termion;

use super::Position;

pub struct Blademaster;

impl<'a> System<'a> for Blademaster {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        loop {
            print!(
                "{}{}{}",
                termion::clear::All,
                termion::cursor::Hide,
                termion::cursor::Goto(1, 1)
            );
            for position in position.join() {
                print!("{}", termion::cursor::Goto(position.x(), position.y()));
                println!("$");
            }
            thread::sleep(Duration::from_millis(100));
        }
    }
}
