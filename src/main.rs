use legion::prelude::*;

use tui::style::Color;

use blademaster::*;

fn main() {
    let universe = Universe::new();
    let mut world = universe.create_world();

    let positions = vec![
        (
            GameCell::new(
                4,
                3,
                CellKind::EdgedWeapon,
                "sword",
                Color::Blue,
                CellAccess::Takeable,
            ),
            CellVisibility::Unvisited,
        ),
        (
            GameCell::new(
                7,
                2,
                CellKind::SoftArmor,
                "leather armor",
                Color::Rgb(150, 75, 0),
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
                        x as i32 + 10,
                        y as i32 + 5,
                        CellKind::Wall,
                        "wall",
                        Color::Gray,
                        CellAccess::Impassable,
                    ),
                    CellVisibility::Unvisited,
                ));
            } else {
                positions.push((
                    GameCell::new(
                        x as i32 + 10,
                        y as i32 + 5,
                        CellKind::Floor,
                        "floor",
                        Color::Gray,
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
                x as i32,
                17,
                CellKind::Wall,
                "wall",
                Color::Gray,
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
                -17,
                y as i32,
                CellKind::Tunnel,
                "tunnel",
                Color::Gray,
                CellAccess::Static,
            ),
            CellVisibility::Unvisited,
        ));
    }
    world.insert((), positions.into_iter());

    let tui_sys = Box::new(|world: &mut World| {
        TuiSystem::run(world);
    });

    let mut schedule = Schedule::builder().add_thread_local_fn(tui_sys).build();

    schedule.execute(&mut world);
}
