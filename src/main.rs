use bracket_lib::prelude::*;

use blademaster::State;

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_advanced_input(true)
        .with_automatic_console_resize(true)
        .with_title("Blademaster")
        .build()?;
    let gs = State::new(80, 50);

    main_loop(ctx, gs)
}
