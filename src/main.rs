use specs::prelude::*;

use blademaster::*;

fn main() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(TuiSystem, "tui_system", &[])
        .build();
    dispatcher.setup(&mut world);

    for x in 0..100 {
        world
            .create_entity()
            .with(GameCell::new('#', x, 25))
            .build();
    }
    for y in -50..0 {
        for x in -100..0 {
            world
                .create_entity()
                .with(GameCell::new('#', x, y * 2))
                .build();
        }
    }

    world.create_entity().with(Inventory::new()).build();
    world.create_entity().with(GameCell::new('#', 5, 6)).build();
    world.create_entity().with(GameCell::new('#', 4, 3)).build();

    dispatcher.dispatch(&world);
    world.maintain();
}
