use legion::prelude::*;

use blademaster::*;

fn main() {
    let universe = Universe::new();
    let mut world = universe.create_world();

    let positions = vec![(GameCell::new(4, 3, '#'),)];
    world.insert((), positions.into_iter());

    let mut positions = Vec::with_capacity(1000);
    for y in 0..100 {
        for x in 0..100 {
            positions.push((GameCell::new(-x, -y, '#'),));
        }
    }
    world.insert((), positions.into_iter());

    TuiSystem::run(&mut world);
}
