use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use crate::World;
use bracket_lib::prelude::*;

struct State {
    world: Arc<Mutex<World>>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        for (y, rows) in self.world.lock().unwrap().data.iter().enumerate() {
            for (x, tile) in rows.iter().enumerate() {
                match tile {
                    crate::Tile::Robot => ctx.print(x, y, "R"),
                    crate::Tile::Food => ctx.print(x, y, "F"),
                    crate::Tile::Empty => ctx.print_color(x, y, GRAY3, BLACK, "."),
                }
            }
        }
        ctx.print(0, 99, "Hello Bracket World");
    }
}

/// Utility function for drawing the world
pub fn draw_world(world: &Arc<Mutex<World>>) {
    let context = BTermBuilder::simple(crate::WORLD_WIDTH, crate::WORLD_HEIGHT)
        .unwrap()
        .with_title("Robofood")
        .build()
        .unwrap();

    let gs: State = State {
        world: world.clone(),
    };
    main_loop(context, gs).expect("error while rendering world")
}
