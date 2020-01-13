use legion::prelude::*;

use blademaster::*;

fn main() {
    let universe = Universe::new();
    let mut world = universe.create_world();

    let positions = vec![(GameCell::new(4, 3, '#'),)];
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
                positions.push((GameCell::new(x as i16 + 10, y as i16 + 5, '#'),));
            }
        }
    }
    world.insert((), positions.into_iter());

    TuiSystem::run(&mut world);
}
