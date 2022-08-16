use crate::World;
use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(0, 99, "Hello Bracket World");
    }
}

pub fn draw_world(world: &World) {
    let context = BTermBuilder::simple(crate::WORLD_WIDTH + 1, crate::WORLD_HEIGHT + 1)
        .unwrap()
        .with_title("Hello Minimal Bracket World")
        .build()
        .unwrap();

    let gs: State = State {};
    main_loop(context, gs).expect("error while rendering world")
}
