use std::{
    io::{stdin, stdout},
    process,
};

use rayon::prelude::*;

use specs::prelude::*;

use termion::{
    self, cursor,
    event::{Event, Key},
    input::TermRead,
    raw::IntoRawMode,
};

use tui::{
    backend::TermionBackend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, List, Text, Widget},
    Terminal,
};

use crate::{types::GameEvents, GameCell, Inventory};

pub struct TuiSystem;

impl<'a> System<'a> for TuiSystem {
    type SystemData = (WriteStorage<'a, Inventory>, WriteStorage<'a, GameCell>);

    fn run(&mut self, (mut inventory, mut gamecells): Self::SystemData) {
        let mut terminal =
            Terminal::new(TermionBackend::new(stdout().into_raw_mode().unwrap())).unwrap();
        terminal.hide_cursor().unwrap();
        terminal.clear().unwrap();

        let term_width = terminal.size().unwrap().width;
        let term_height = terminal.size().unwrap().height;
        let canvas_width = term_width - 25;
        let canvas_height = term_height - 8;
        let player_x = (term_width as f64 / 2.0).round();
        let player_y = (canvas_height as f64 / 2.0).round();

        println!(
            "{}Welcome to {}{}Blademaster{}{}{}",
            cursor::Goto(term_width / 2 - 15, 1),
            termion::color::Fg(termion::color::Blue),
            termion::style::Bold,
            termion::color::Fg(termion::color::Reset),
            termion::style::Reset,
            cursor::Goto(1, 1)
        );

        let mut game_events = GameEvents::new();

        for event in stdin().events() {
            match event.unwrap() {
                Event::Key(Key::Up) => {
                    let mut collided = false;
                    for gamecell in gamecells.join() {
                        if (player_x - gamecell.x() as f64).abs() < 1.0
                            && (player_y - (gamecell.y() - 1) as f64).abs() < 1.0
                        {
                            game_events.post_wall_event();
                            collided = true;
                            break;
                        }
                    }
                    if !collided {
                        (&mut gamecells).par_join().for_each(|gamecell| {
                            gamecell.move_up();
                        });
                    }
                }
                Event::Key(Key::Down) => {
                    let mut collided = false;
                    for gamecell in gamecells.join() {
                        if (player_x - gamecell.x() as f64).abs() < 1.0
                            && (player_y - (gamecell.y() + 1) as f64).abs() < 1.0
                        {
                            game_events.post_wall_event();
                            collided = true;
                            break;
                        }
                    }
                    if !collided {
                        (&mut gamecells).par_join().for_each(|gamecell| {
                            gamecell.move_down();
                        });
                    }
                }
                Event::Key(Key::Left) => {
                    let mut collided = false;
                    for gamecell in gamecells.join() {
                        if (player_x - (gamecell.x() + 1) as f64).abs() < 1.0
                            && (player_y - gamecell.y() as f64).abs() < 1.0
                        {
                            game_events.post_wall_event();
                            collided = true;
                            break;
                        }
                    }
                    if !collided {
                        (&mut gamecells).par_join().for_each(|gamecell| {
                            gamecell.move_right();
                        });
                    }
                }
                Event::Key(Key::Right) => {
                    let mut collided = false;
                    for gamecell in gamecells.join() {
                        if (player_x - (gamecell.x() - 1) as f64).abs() < 1.0
                            && (player_y - gamecell.y() as f64).abs() < 1.0
                        {
                            game_events.post_wall_event();
                            collided = true;
                            break;
                        }
                    }
                    if !collided {
                        (&mut gamecells).par_join().for_each(|gamecell| {
                            gamecell.move_left();
                        });
                    }
                }
                Event::Key(Key::Char('q')) => {
                    terminal.clear().unwrap();
                    terminal.show_cursor().unwrap();
                    process::exit(1);
                }
                _ => (),
            }

            terminal
                .draw(|mut f| {
                    let chunks = Layout::default()
                        .margin(0)
                        .direction(Direction::Vertical)
                        .constraints(
                            [
                                Constraint::Length(canvas_height + 1),
                                Constraint::Length(term_height - canvas_height - 2),
                            ]
                            .as_ref(),
                        )
                        .split(f.size());
                    let top_chunks = Layout::default()
                        .margin(0)
                        .direction(Direction::Horizontal)
                        .constraints(
                            [
                                Constraint::Length(canvas_width + 1),
                                Constraint::Length(term_width - canvas_width - 2),
                            ]
                            .as_ref(),
                        )
                        .split(chunks[0]);
                    let bottom_chunks = Layout::default()
                        .margin(0)
                        .direction(Direction::Horizontal)
                        .constraints(
                            [Constraint::Percentage(70), Constraint::Percentage(30)].as_ref(),
                        )
                        .split(chunks[1]);
                    Canvas::default()
                        .block(Block::default().borders(Borders::ALL).title("Game"))
                        .paint(|ctx| {
                            ctx.print(player_x, player_y, "@", Color::Rgb(0, 255, 0));
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
                        .x_bounds([2.0, canvas_width as f64])
                        .y_bounds([2.0, canvas_height as f64])
                        .render(&mut f, top_chunks[0]);
                    List::new(
                        vec![
                            Text::styled("Fake item\n", Style::default().fg(Color::Blue)),
                            Text::styled("A rock\n", Style::default().fg(Color::Blue)),
                        ]
                        .into_iter(),
                    )
                    .block(Block::default().borders(Borders::ALL).title("Inventory"))
                    .start_corner(Corner::TopLeft)
                    .render(&mut f, top_chunks[1]);
                    List::new(game_events.events().clone().into_iter())
                        .block(Block::default().borders(Borders::ALL).title("Events"))
                        .start_corner(Corner::TopLeft)
                        .render(&mut f, bottom_chunks[0]);
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Player")
                        .render(&mut f, bottom_chunks[1]);
                })
                .unwrap();
        }
    }
}
