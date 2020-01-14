use legion::prelude::*;

use tui::style::Color;

use blademaster::*;

fn main() {
    let universe = Universe::new();
    let mut world = universe.create_world();

    let positions = vec![
        (GameCell::new(
            4,
            3,
            CellKind::EdgedWeapon,
            "sword",
            Color::Blue,
            CellAccess::Takeable,
        ),),
        (GameCell::new(
            7,
            2,
            CellKind::SoftArmor,
            "leather armor",
            Color::Rgb(150, 75, 0),
            CellAccess::Takeable,
        ),),
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
                positions.push((GameCell::new(
                    x as i16 + 10,
                    y as i16 + 5,
                    CellKind::Wall,
                    "wall",
                    Color::Gray,
                    CellAccess::Impassable,
                ),));
            }
        }
    }
    world.insert((), positions.into_iter());

    TuiSystem::run(&mut world);
}
