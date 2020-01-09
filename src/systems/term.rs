use std::{
    convert::TryInto,
    io::{stdin, stdout, Write},
    process,
};

use specs::prelude::*;

use termion::{
    self, cursor,
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
};

use term_size;

use crate::Block;

pub struct TermionSystem;

impl<'a> System<'a> for TermionSystem {
    type SystemData = WriteStorage<'a, Block>;

    fn run(&mut self, mut blocks: Self::SystemData) {
        println!("Press any key...");

        let mut stdout = stdout().into_raw_mode().unwrap();

        let (term_width, term_height) = if let Some((w, h)) = term_size::dimensions() {
            (w.try_into().unwrap_or(1), h.try_into().unwrap_or(1))
        } else {
            (1, 1)
        };

        print!("{}", cursor::Hide,);

        for event in stdin().events() {
            match event.unwrap() {
                Event::Key(Key::Up) => {
                    for block in (&mut blocks).join() {
                        block.move_down();
                    }
                }
                Event::Key(Key::Down) => {
                    for block in (&mut blocks).join() {
                        block.move_up();
                    }
                }
                Event::Key(Key::Left) => {
                    for block in (&mut blocks).join() {
                        block.move_right();
                    }
                }
                Event::Key(Key::Right) => {
                    for block in (&mut blocks).join() {
                        block.move_left();
                    }
                }
                Event::Key(Key::Char('q')) => {
                    process::exit(1);
                }
                _ => (),
            }

            print!(
                "{}{}@",
                termion::clear::All,
                cursor::Goto(term_width / 2, term_height / 2)
            );

            for block in blocks.join() {
                print!("{}{}", cursor::Goto(block.x(), block.y()), block.symbol());
            }

            print!("{}", cursor::Goto(1, 1));
            for _ in 0..term_width {
                print!("-");
            }
            print!("{}", cursor::Goto(1, term_height));
            for _ in 0..term_width {
                print!("-");
            }
            for y in 2..term_height {
                print!("{}|{}|", cursor::Goto(0, y), cursor::Goto(term_width, y));
            }
            stdout.flush().unwrap();
        }
    }
}
