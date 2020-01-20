use std::{
    io::{stdout, Write},
    ops::Deref,
    process,
};

use legion::prelude::*;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Corner, Direction, Layout},
    style::Color,
    widgets::{canvas::Canvas, Block, Borders, List, Widget},
    Terminal,
};

use crate::{CellAccess, CellKind, CellVisibility, GameCell, GameEvents, Inventory, Player};

pub struct TuiSystem;

impl TuiSystem {
    pub fn run(world: &mut World) {
        let read_query = <(Read<GameCell>,)>::query();
        let write_query = <(legion::query::Write<GameCell>,)>::query();

        enable_raw_mode().unwrap();

        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();

        let mut terminal = Terminal::new(CrosstermBackend::new(stdout)).unwrap();
        terminal.hide_cursor().unwrap();
        terminal.clear().unwrap();

        let term_width = terminal.size().unwrap().width;
        let term_height = terminal.size().unwrap().height;
        let canvas_width = term_width - 25;
        let canvas_height = term_height - 8;

        let player = Player::new(
            (canvas_width as f64 / 2.0).round(),
            (canvas_height as f64 / 2.0).round(),
        );

        let mut game_events = GameEvents::new();

        let mut inventory = Inventory::new();

        loop {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Up => {
                        let mut collided = false;
                        for (gamecell,) in read_query.iter_immutable(world) {
                            if gamecell.access() == CellAccess::Impassable
                                && (player.x() - gamecell.x() as f64).abs() < 1.0
                                && (player.y() - (gamecell.y() - 1) as f64).abs() < 1.0
                            {
                                game_events.post_event(
                                    format!(
                                        "You ran into the {}.{space:>width$}",
                                        gamecell.name(),
                                        space = " ",
                                        width = canvas_width as usize / 2,
                                    ),
                                    Color::Blue,
                                );
                                collided = true;
                                break;
                            }
                        }
                        if !collided {
                            write_query.par_for_each(world, {
                                |(mut gamecell,)| {
                                    if gamecell.inside(
                                        player.x() as u16 - 5,
                                        player.y() as u16 - 5,
                                        player.x() as u16 + 5,
                                        player.y() as u16 + 5,
                                    ) {
                                        gamecell.set_visible(CellVisibility::Visible);
                                    } else if gamecell.visible() == CellVisibility::Visible {
                                        gamecell.set_visible(CellVisibility::Dark);
                                    }
                                    gamecell.move_up();
                                }
                            });
                        }
                    }
                    KeyCode::Down => {
                        let mut collided = false;
                        for (gamecell,) in read_query.iter_immutable(world) {
                            if gamecell.access() == CellAccess::Impassable
                                && (player.x() - gamecell.x() as f64).abs() < 1.0
                                && (player.y() - (gamecell.y() + 1) as f64).abs() < 1.0
                            {
                                game_events.post_event(
                                    format!(
                                        "You ran into the {}.{space:>width$}",
                                        gamecell.name(),
                                        space = " ",
                                        width = canvas_width as usize / 2,
                                    ),
                                    Color::Blue,
                                );
                                collided = true;
                                break;
                            }
                        }
                        if !collided {
                            write_query.par_for_each(world, {
                                |(mut gamecell,)| {
                                    if gamecell.inside(
                                        player.x() as u16 - 5,
                                        player.y() as u16 - 5,
                                        player.x() as u16 + 5,
                                        player.y() as u16 + 5,
                                    ) {
                                        gamecell.set_visible(CellVisibility::Visible);
                                    } else if gamecell.visible() == CellVisibility::Visible {
                                        gamecell.set_visible(CellVisibility::Dark);
                                    }
                                    gamecell.move_down();
                                }
                            });
                        }
                    }
                    KeyCode::Left => {
                        let mut collided = false;
                        for (gamecell,) in read_query.iter_immutable(world) {
                            if gamecell.access() == CellAccess::Impassable
                                && (player.x() - (gamecell.x() + 1) as f64).abs() < 1.0
                                && (player.y() - gamecell.y() as f64).abs() < 1.0
                            {
                                game_events.post_event(
                                    format!(
                                        "You ran into the {}.{space:>width$}",
                                        gamecell.name(),
                                        space = " ",
                                        width = canvas_width as usize / 2,
                                    ),
                                    Color::Blue,
                                );
                                collided = true;
                                break;
                            }
                        }
                        if !collided {
                            write_query.par_for_each(world, {
                                |(mut gamecell,)| {
                                    if gamecell.inside(
                                        player.x() as u16 - 5,
                                        player.y() as u16 - 5,
                                        player.x() as u16 + 5,
                                        player.y() as u16 + 5,
                                    ) {
                                        gamecell.set_visible(CellVisibility::Visible);
                                    } else if gamecell.visible() == CellVisibility::Visible {
                                        gamecell.set_visible(CellVisibility::Dark);
                                    }
                                    gamecell.move_right();
                                }
                            });
                        }
                    }
                    KeyCode::Right => {
                        let mut collided = false;
                        for (gamecell,) in read_query.iter_immutable(world) {
                            if gamecell.access() == CellAccess::Impassable
                                && (player.x() - (gamecell.x() - 1) as f64).abs() < 1.0
                                && (player.y() - gamecell.y() as f64).abs() < 1.0
                            {
                                game_events.post_event(
                                    format!(
                                        "You ran into the {}.{space:>width$}",
                                        gamecell.name(),
                                        space = " ",
                                        width = canvas_width as usize / 2,
                                    ),
                                    Color::Blue,
                                );
                                collided = true;
                                break;
                            }
                        }
                        if !collided {
                            write_query.par_for_each(world, {
                                |(mut gamecell,)| {
                                    if gamecell.inside(
                                        player.x() as u16 - 5,
                                        player.y() as u16 - 5,
                                        player.x() as u16 + 5,
                                        player.y() as u16 + 5,
                                    ) {
                                        gamecell.set_visible(CellVisibility::Visible);
                                    } else if gamecell.visible() == CellVisibility::Visible {
                                        gamecell.set_visible(CellVisibility::Dark);
                                    }
                                    gamecell.move_left();
                                }
                            });
                        }
                    }
                    KeyCode::Char('q') => {
                        terminal.clear().unwrap();
                        terminal.show_cursor().unwrap();
                        disable_raw_mode().unwrap();
                        execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
                        process::exit(1);
                    }
                    _ => (),
                }
            }

            let read_query = <(Read<GameCell>,)>::query();

            let mut taken = None;
            for (entity, (gamecell,)) in read_query.iter_entities_immutable(world) {
                if gamecell.access() == CellAccess::Takeable
                    && gamecell.inside(1, 1, term_width, term_height)
                    && (player.x() - gamecell.x() as f64).abs() < 1.0
                    && (player.y() - gamecell.y() as f64).abs() < 1.0
                {
                    game_events.post_event(
                        format!(
                            "You now have the {}.{space:>width$}",
                            gamecell.name(),
                            space = " ",
                            width = canvas_width as usize / 2,
                        ),
                        Color::Green,
                    );
                    inventory.take(gamecell.deref().clone());
                    taken = Some(entity);
                    break;
                }
            }
            if let Some(entity) = taken {
                world.delete(entity);
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
                            for (gamecell,) in read_query.iter_immutable(world) {
                                if gamecell.visible() != CellVisibility::Unvisited
                                    && gamecell.inside(1, 1, term_width, term_height)
                                {
                                    let symbol = match gamecell.kind() {
                                        CellKind::SoftArmor => "(",
                                        CellKind::HardArmor => "[",
                                        CellKind::BluntWeapon => "\\",
                                        CellKind::EdgedWeapon => "|",
                                        CellKind::PointedWeapon => "/",
                                        CellKind::RangedWeapon => "}",
                                        CellKind::ClosedDoor => "+",
                                        CellKind::OpenedDoor => "'",
                                        CellKind::Wall => "#",
                                        CellKind::Floor => ".",
                                    };
                                    if gamecell.visible() == CellVisibility::Visible {
                                        ctx.print(
                                            gamecell.x() as f64,
                                            gamecell.y() as f64,
                                            symbol,
                                            gamecell.color(),
                                        );
                                    } else {
                                        ctx.print(
                                            gamecell.x() as f64,
                                            gamecell.y() as f64,
                                            symbol,
                                            Color::DarkGray,
                                        );
                                    }
                                }
                            }
                            ctx.print(player.x(), player.y(), "@", Color::Rgb(0, 255, 0));
                        })
                        .x_bounds([2.0, canvas_width as f64])
                        .y_bounds([2.0, canvas_height as f64])
                        .render(&mut f, top_chunks[0]);
                    List::new(inventory.list().into_iter())
                        .block(Block::default().borders(Borders::ALL).title("Inventory"))
                        .start_corner(Corner::TopLeft)
                        .render(&mut f, top_chunks[1]);
                    List::new(game_events.events().clone().into_iter())
                        .block(Block::default().borders(Borders::ALL).title("Events"))
                        .start_corner(Corner::TopLeft)
                        .render(&mut f, bottom_chunks[0]);
                    List::new(player.list().into_iter())
                        .block(Block::default().borders(Borders::ALL).title("Player"))
                        .start_corner(Corner::TopLeft)
                        .render(&mut f, bottom_chunks[1]);
                })
                .unwrap();
        }
    }
}
