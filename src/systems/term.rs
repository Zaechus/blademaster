use std::{
    io::{stdout, Write},
    ops::Deref,
    process,
};

use legion::{prelude::*, query};

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
        let read_query = <(Read<GameCell>, Read<CellVisibility>)>::query();
        let write_query = <(Read<GameCell>, query::Write<CellVisibility>)>::query();

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

        let player = Player::new(canvas_width as i32 / 2, canvas_height as i32 / 2);

        let mut game_events = GameEvents::new();

        let mut inventory = Inventory::new();

        let mut offset_x = 0;
        let mut offset_y = 0;

        loop {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Up => {
                        let mut collided = false;
                        for (gamecell, _) in read_query.iter_immutable(world) {
                            if gamecell.access() == CellAccess::Impassable
                                && player.x() == gamecell.x() + offset_x
                                && player.y() == (gamecell.y() - 1) + offset_y
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
                            offset_y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        let mut collided = false;
                        for (gamecell, _) in read_query.iter_immutable(world) {
                            if gamecell.access() == CellAccess::Impassable
                                && player.x() == gamecell.x() + offset_x
                                && player.y() == (gamecell.y() + 1) + offset_y
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
                            offset_y += 1;
                        }
                    }
                    KeyCode::Left => {
                        let mut collided = false;
                        for (gamecell, _) in read_query.iter_immutable(world) {
                            if gamecell.access() == CellAccess::Impassable
                                && player.x() == (gamecell.x() + 1) + offset_x
                                && player.y() == gamecell.y() + offset_y
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
                            offset_x += 1;
                        }
                    }
                    KeyCode::Right => {
                        let mut collided = false;
                        for (gamecell, _) in read_query.iter_immutable(world) {
                            if gamecell.access() == CellAccess::Impassable
                                && player.x() == (gamecell.x() - 1) + offset_x
                                && player.y() == gamecell.y() + offset_y
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
                            offset_x -= 1;
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

            for (gamecell, mut visible) in write_query.iter(world) {
                if gamecell.inside(
                    player.x() - 4,
                    player.y() - 4,
                    player.x() + 4,
                    player.y() + 4,
                    offset_x,
                    offset_y,
                ) {
                    *visible = CellVisibility::Visible;
                } else if *visible == CellVisibility::Visible {
                    *visible = CellVisibility::Dark;
                }
            }

            let mut taken = None;
            for (entity, (gamecell, _)) in read_query.iter_entities_immutable(world) {
                if gamecell.access() == CellAccess::Takeable
                    && gamecell.inside(
                        1,
                        1,
                        term_width as i32,
                        term_height as i32,
                        offset_x,
                        offset_y,
                    )
                    && player.x() == gamecell.x() + offset_x
                    && player.y() == gamecell.y() + offset_y
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
                            for (gamecell, visible) in read_query.iter_immutable(world) {
                                if *visible != CellVisibility::Unvisited
                                    && gamecell.inside(
                                        1,
                                        1,
                                        term_width as i32,
                                        term_height as i32,
                                        offset_x,
                                        offset_y,
                                    )
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
                                        CellKind::Tunnel => "░",
                                        CellKind::Floor => ".",
                                    };
                                    if *visible == CellVisibility::Visible {
                                        ctx.print(
                                            (gamecell.x() + offset_x) as f64,
                                            (gamecell.y() + offset_y) as f64,
                                            symbol,
                                            gamecell.color(),
                                        );
                                    } else {
                                        ctx.print(
                                            (gamecell.x() + offset_x) as f64,
                                            (gamecell.y() + offset_y) as f64,
                                            symbol,
                                            Color::DarkGray,
                                        );
                                    }
                                }
                            }
                            ctx.print(
                                player.x() as f64,
                                player.y() as f64,
                                "@",
                                Color::Rgb(0, 255, 0),
                            );
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
