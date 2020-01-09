use std::{
    io::{stdin, stdout},
    process,
};

use rayon::prelude::*;

use specs::prelude::*;

use termion::{
    self,
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
};

use tui::{
    backend::TermionBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph, Text, Widget},
    Terminal,
};

// use crate::util::{Config, Event, Events};
use crate::GameCell;

pub struct TuiSystem;

impl<'a> System<'a> for TuiSystem {
    type SystemData = WriteStorage<'a, GameCell>;

    fn run(&mut self, mut gamecells: Self::SystemData) {
        println!("Press any key...");

        let mut terminal =
            Terminal::new(TermionBackend::new(stdout().into_raw_mode().unwrap())).unwrap();
        terminal.hide_cursor().unwrap();
        terminal.clear().unwrap();

        let term_width = terminal.size().unwrap().width;
        let term_height = terminal.size().unwrap().height;
        let canvas_height = term_height - 8;

        for event in stdin().events() {
            match event.unwrap() {
                Event::Key(Key::Up) => {
                    (&mut gamecells).par_join().for_each(|gamecell| {
                        gamecell.move_up();
                    });
                }
                Event::Key(Key::Down) => {
                    (&mut gamecells).par_join().for_each(|gamecell| {
                        gamecell.move_down();
                    });
                }
                Event::Key(Key::Left) => {
                    (&mut gamecells).par_join().for_each(|gamecell| {
                        gamecell.move_right();
                    });
                }
                Event::Key(Key::Right) => {
                    (&mut gamecells).par_join().for_each(|gamecell| {
                        gamecell.move_left();
                    });
                }
                Event::Key(Key::Char('q')) => {
                    process::exit(1);
                }
                _ => (),
            }

            terminal
                .draw(|mut f| {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .margin(0)
                        .constraints(
                            [
                                Constraint::Length(canvas_height + 1),
                                Constraint::Length(term_height - canvas_height - 2),
                            ]
                            .as_ref(),
                        )
                        .split(f.size());
                    Canvas::default()
                        .block(Block::default().borders(Borders::ALL).title("Game"))
                        .paint(|ctx| {
                            ctx.print(
                                (term_width as f64 / 2.0).round(),
                                (canvas_height as f64 / 2.0).round(),
                                "@",
                                Color::Green,
                            );
                            for gamecell in gamecells.join() {
                                if gamecell.inside(1, 1, term_width, term_height) {
                                    ctx.print(
                                        gamecell.x() as f64,
                                        gamecell.y() as f64,
                                        "#",
                                        Color::Gray,
                                    );
                                }
                            }
                        })
                        .x_bounds([2.0, (term_width - 1) as f64])
                        .y_bounds([2.0, canvas_height as f64])
                        .render(&mut f, chunks[0]);
                    let text = vec![
                        Text::styled("Sample event.\n", Style::default().fg(Color::Blue)),
                        Text::styled(
                            "I'M A GOOFY GOOBER!!! I'M A GOOFY GOOBER!!! I'M A GOOFY GOOBER!!! I'M A GOOFY GOOBER!!! I'M A GOOFY GOOBER!!!\n",
                            Style::default().fg(Color::Blue),
                        ),
                    ];
                    Paragraph::new(
                        text.iter(),
                    )
                    .block(Block::default().borders(Borders::ALL).title("Events"))
                    .alignment(Alignment::Left)
                    .wrap(true)
                    .render(&mut f, chunks[1]);
                })
                .unwrap();
        }
    }
}
