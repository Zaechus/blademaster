use bracket_lib::prelude::*;

use legion::prelude::*;

use crate::{
    components::{CellVisibility, GameCell},
    types::{CellAccess, CellKind, Player},
};

const GRAY: (u8, u8, u8) = (150, 150, 150);
const DARK_GRAY: (u8, u8, u8) = (100, 100, 100);

#[derive(Clone, Debug)]
pub enum CurrentState {
    Menu,
    Playing,
    Quitting,
}

add_wasm_support!();

pub struct State {
    curr_state: CurrentState,
    world: World,
    player: Player,
    window_size: (u32, u32),
    tic: u8,
    offset: (i32, i32),
    mouse: Point,
    mouse_click: Option<(usize, bool)>,
    mouse_pressed: (usize, bool),
    cursor: String,
}

impl State {
    pub fn new(w: u32, h: u32) -> Self {
        let universe = Universe::new();
        let mut world = universe.create_world();

        let positions = vec![
            (
                GameCell::new(
                    Point::new(4, 3),
                    CellKind::EdgedWeapon,
                    "sword",
                    RGB::from_u8(0, 0, 255),
                    CellAccess::Takeable,
                ),
                CellVisibility::Unvisited,
            ),
            (
                GameCell::new(
                    Point::new(7, 2),
                    CellKind::SoftArmor,
                    "leather armor",
                    RGB::from_u8(150, 75, 0),
                    CellAccess::Takeable,
                ),
                CellVisibility::Unvisited,
            ),
        ];
        world.insert((), positions.into_iter());

        let map = vec![
            "#.########",
            "#.##......",
            "#.##.#####",
            "#.##...###",
            "#.####.###",
            "#......###",
            "##########",
        ];
        let mut positions = Vec::with_capacity(70);
        for (y, row) in map.iter().rev().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    positions.push((
                        GameCell::new(
                            Point::new(x as i32 + 10, y as i32 + 5),
                            CellKind::Wall,
                            "wall",
                            RGB::named(GRAY),
                            CellAccess::Impassable,
                        ),
                        CellVisibility::Unvisited,
                    ));
                } else {
                    positions.push((
                        GameCell::new(
                            Point::new(x as i32 + 10, y as i32 + 5),
                            CellKind::Floor,
                            "floor",
                            RGB::named(GRAY),
                            CellAccess::Static,
                        ),
                        CellVisibility::Unvisited,
                    ));
                }
            }
        }
        world.insert((), positions.into_iter());

        let mut positions = Vec::with_capacity(70);
        for x in 0..100 {
            positions.push((
                GameCell::new(
                    Point::new(x as i32, 17),
                    CellKind::Wall,
                    "wall",
                    RGB::from_u8(150, 150, 150),
                    CellAccess::Impassable,
                ),
                CellVisibility::Unvisited,
            ));
        }
        world.insert((), positions.into_iter());

        let mut positions = Vec::with_capacity(70);
        for y in 0..100 {
            positions.push((
                GameCell::new(
                    Point::new(-17, y as i32),
                    CellKind::Tunnel,
                    "tunnel",
                    RGB::from_u8(150, 150, 150),
                    CellAccess::Static,
                ),
                CellVisibility::Unvisited,
            ));
        }
        world.insert((), positions.into_iter());

        Self {
            curr_state: CurrentState::Menu,
            world,
            player: Player::new(Point::new(w as i32 / 2, h as i32 / 2)),
            window_size: (w, h),
            tic: 0,
            offset: (0, 0),
            mouse: Point::new(0, 0),
            mouse_click: None,
            mouse_pressed: (0, false),
            cursor: String::from("<"),
        }
    }

    fn menu_state(&mut self, ctx: &mut BTerm) {
        ctx.print_centered(self.window_size.1 as i32 / 2 - 1, "Blademaster");
        ctx.print_centered(
            self.window_size.1 as i32 / 2 + 1,
            "Press the spacebar to start",
        );

        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.curr_state = CurrentState::Playing;
        }
    }

    fn play_state(&mut self, ctx: &mut BTerm) {
        // self.print_grid(ctx);

        ctx.print_color(
            self.mouse.x,
            self.mouse.y,
            RGB::named((0, 155 + self.tic, 0)),
            RGB::new(),
            &self.cursor,
        );

        self.update_cells(ctx);

        ctx.print_color(
            self.player.x(),
            self.player.y(),
            RGB::named((0, 255, 0)),
            RGB::new(),
            "@",
        );

        self.player.default_sight();

        match self.mouse_click {
            _ => (),
        }

        self.key_input(ctx);
    }

    fn key_input(&mut self, ctx: &mut BTerm) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Up => self.offset.1 += 1,
                VirtualKeyCode::Down => self.offset.1 -= 1,
                VirtualKeyCode::Left => self.offset.0 += 1,
                VirtualKeyCode::Right => self.offset.0 -= 1,
                VirtualKeyCode::End => self.curr_state = CurrentState::Quitting,
                _ => (),
            }
        }
    }

    fn update_cells(&mut self, ctx: &mut BTerm) {
        let query = <(Read<GameCell>, Read<CellVisibility>)>::query();

        for (cell, _) in query.iter(&mut self.world) {
            if Rect::with_exact(
                -self.offset.0,
                -self.offset.1,
                self.window_size.0 as i32 - self.offset.0,
                self.window_size.1 as i32 - self.offset.1,
            )
            .point_in_rect(cell.point())
            {
                // if *visible != CellVisibility::Unvisited
                //     && cell.inside(
                //         1,
                //         1,
                //         self.window_size.0 as i32,
                //         self.window_size.1 as i32,
                //         self.offset.0,
                //         self.offset.1,
                //     )
                // {
                if cell.inside(
                    self.player.x() - self.player.sight().0,
                    self.player.y() - self.player.sight().1,
                    self.player.x() + self.player.sight().2,
                    self.player.y() + self.player.sight().3,
                    self.offset.0,
                    self.offset.1,
                ) {
                    ctx.print_color(
                        cell.x() + self.offset.0,
                        cell.y() + self.offset.1,
                        if self.mouse.x - self.offset.0 == cell.x()
                            && self.mouse.y - self.offset.1 == cell.y()
                        {
                            cell.color_bright()
                        } else {
                            cell.color()
                        },
                        cell.bg_color(),
                        &cell.symbol().to_string(),
                    );
                } else {
                    ctx.print_color(
                        cell.x() + self.offset.0,
                        cell.y() + self.offset.1,
                        RGB::named(DARK_GRAY),
                        cell.bg_color(),
                        &cell.symbol().to_string(),
                    );
                }
                // }
            }
        }
    }

    fn quit_state(&mut self, ctx: &mut BTerm) {
        ctx.print(5, 5, "Are you sure you want to quit? (y/n)");

        if let Some(VirtualKeyCode::Y) = ctx.key {
            ctx.quit();
        } else if let Some(VirtualKeyCode::N) = ctx.key {
            self.curr_state = CurrentState::Playing;
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        let mut input = INPUT.lock();

        input.for_each_message(|event| match event {
            BEvent::MouseClick { button, pressed } => self.mouse_click = Some((button, pressed)),
            BEvent::MouseButtonUp { button } => self.mouse_pressed = (button, false),
            BEvent::MouseButtonDown { button } => self.mouse_pressed = (button, true),
            _ => (),
        });

        self.tic += 4;
        if self.tic > 99 {
            self.tic = 0;
        }

        self.mouse = ctx.mouse_point();

        match self.curr_state {
            CurrentState::Menu => self.menu_state(ctx),
            CurrentState::Playing => self.play_state(ctx),
            CurrentState::Quitting => self.quit_state(ctx),
        }

        // let read_query = <(Read<GameCell>, Read<CellVisibility>)>::query();
        // let write_query = <(Read<GameCell>, Write<CellVisibility>)>::query();

        // let mut game_events = GameEvents::new();

        // let mut inventory = Inventory::new();

        //     if let Event::Key(key) = event::read().unwrap() {
        //         match key.code {
        //             KeyCode::Up => {
        //                 let mut collided = false;
        //                 for (gamecell, _) in read_query.iter_immutable(world) {
        //                     if gamecell.access() == CellAccess::Impassable
        //                         && player.x() == gamecell.x() + self.offset.0
        //                         && player.y() == (gamecell.y() - 1) + self.offset.1
        //                     {
        //                         game_events.post_event(
        //                             format!(
        //                                 "You ran into the {}.{space:>width$}",
        //                                 gamecell.name(),
        //                                 space = " ",
        //                                 width = canvas_width as usize / 2,
        //                             ),
        //                             Color::Blue,
        //                         );
        //                         collided = true;
        //                         break;
        //                     }
        //                 }
        //                 if !collided {
        //                     self.offset.1 -= 1;
        //                 }
        //             }
        //             KeyCode::Down => {
        //                 let mut collided = false;
        //                 for (gamecell, _) in read_query.iter_immutable(world) {
        //                     if gamecell.access() == CellAccess::Impassable
        //                         && player.x() == gamecell.x() + self.offset.0
        //                         && player.y() == (gamecell.y() + 1) + self.offset.1
        //                     {
        //                         game_events.post_event(
        //                             format!(
        //                                 "You ran into the {}.{space:>width$}",
        //                                 gamecell.name(),
        //                                 space = " ",
        //                                 width = canvas_width as usize / 2,
        //                             ),
        //                             Color::Blue,
        //                         );
        //                         collided = true;
        //                         break;
        //                     }
        //                 }
        //                 if !collided {
        //                     self.offset.1 += 1;
        //                 }
        //             }
        //             KeyCode::Left => {
        //                 let mut collided = false;
        //                 for (gamecell, _) in read_query.iter_immutable(world) {
        //                     if gamecell.access() == CellAccess::Impassable
        //                         && player.x() == (gamecell.x() + 1) + self.offset.0
        //                         && player.y() == gamecell.y() + self.offset.1
        //                     {
        //                         game_events.post_event(
        //                             format!(
        //                                 "You ran into the {}.{space:>width$}",
        //                                 gamecell.name(),
        //                                 space = " ",
        //                                 width = canvas_width as usize / 2,
        //                             ),
        //                             Color::Blue,
        //                         );
        //                         collided = true;
        //                         break;
        //                     }
        //                 }
        //                 if !collided {
        //                     self.offset.0 += 1;
        //                 }
        //             }
        //             KeyCode::Right => {
        //                 let mut collided = false;
        //                 for (gamecell, _) in read_query.iter_immutable(world) {
        //                     if gamecell.access() == CellAccess::Impassable
        //                         && player.x() == (gamecell.x() - 1) + self.offset.0
        //                         && player.y() == gamecell.y() + self.offset.1
        //                     {
        //                         game_events.post_event(
        //                             format!(
        //                                 "You ran into the {}.{space:>width$}",
        //                                 gamecell.name(),
        //                                 space = " ",
        //                                 width = canvas_width as usize / 2,
        //                             ),
        //                             Color::Blue,
        //                         );
        //                         collided = true;
        //                         break;
        //                     }
        //                 }
        //                 if !collided {
        //                     self.offset.0 -= 1;
        //                 }
        //             }
        //             KeyCode::Char('q') => {
        //                 terminal.clear().unwrap();
        //                 terminal.show_cursor().unwrap();
        //                 disable_raw_mode().unwrap();
        //                 execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        //                 process::exit(1);
        //             }
        //             _ => (),
        //         }
        //     }

        //     for (gamecell, _) in write_query.iter(world) {
        //         player.reduce_sight(&*gamecell, self.offset.0, self.offset.1);
        //     }

        //     for (gamecell, mut visible) in write_query.iter(world) {
        //         if gamecell.inside(
        //             player.x() - player.sight().0,
        //             player.y() - player.sight().1,
        //             player.x() + player.sight().2,
        //             player.y() + player.sight().3,
        //             self.offset.0,
        //             self.offset.1,
        //         ) {
        //             *visible = CellVisibility::Visible;
        //         } else if *visible == CellVisibility::Visible {
        //             *visible = CellVisibility::Dark;
        //         }
        //     }

        //     let mut taken = None;
        //     for (entity, (gamecell, _)) in read_query.iter_entities_immutable(world) {
        //         if gamecell.access() == CellAccess::Takeable
        //             && gamecell.inside(
        //                 1,
        //                 1,
        //                 term_width as i32,
        //                 term_height as i32,
        //                 self.offset.0,
        //                 self.offset.1,
        //             )
        //             && player.x() == gamecell.x() + self.offset.0
        //             && player.y() == gamecell.y() + self.offset.1
        //         {
        //             game_events.post_event(
        //                 format!(
        //                     "You now have the {}.{space:>width$}",
        //                     gamecell.name(),
        //                     space = " ",
        //                     width = canvas_width as usize / 2,
        //                 ),
        //                 Color::Green,
        //             );
        //             inventory.take(gamecell.deref().clone());
        //             taken = Some(entity);
        //             break;
        //         }
        //     }
        //     if let Some(entity) = taken {
        //         world.delete(entity);
        //     }
        //                     ctx.print(
        //                         player.x() as f64,
        //                         player.y() as f64,
        //                         "@",
        //                         Color::Rgb(0, 255, 0),
        //                     );

        self.mouse_click = None;
    }
}
