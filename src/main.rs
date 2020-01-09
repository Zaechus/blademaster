use specs::prelude::*;

use blademaster::*;

fn main() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(TermionSystem, "termion_system", &[])
        .build();

    dispatcher.setup(&mut world);

    for x in 0..100 {
        world.create_entity().with(Block::new('#', x, 25)).build();
    }

    world.create_entity().with(Block::new('#', 5, 6)).build();
    world.create_entity().with(Block::new('#', 4, 3)).build();

    dispatcher.dispatch(&mut world);
    world.maintain();
}
