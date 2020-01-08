use specs::prelude::*;

use blademaster::*;

fn main() {
    let mut world = World::new();
    world.register::<Position>();

    world.create_entity().with(Position::new(5, 6)).build();
    world.create_entity().with(Position::new(4, 3)).build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(Blademaster, "blademaster", &[])
        .with(InputSys, "input_system", &[])
        .build();

    dispatcher.dispatch(&mut world);
    world.maintain();
}
