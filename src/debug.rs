use std::sync::{Arc, RwLock};

use crate::World;
use bracket_lib::prelude::*;

struct State {
    world: Arc<RwLock<World>>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Render the world
        for (y, rows) in self.world.read().unwrap().data.iter().enumerate() {
            for (x, tile) in rows.iter().enumerate() {
                match tile {
                    crate::Tile::Robot => ctx.print_color(x, y, WHITE, BLACK, "R"),
                    crate::Tile::Food => ctx.print_color(x, y, WHEAT, BLACK, "F"),
                    crate::Tile::Empty => ctx.print_color(x, y, GRAY30, BLACK, "."),
                }
            }
        }

        // Show the state
        let state = self.world.read().unwrap().world_state();
        match state {
            crate::WorldState::FoundFood => ctx.print_color(0, 0, WHITE, BLACK, "Robot: <3 yummy!"),
            crate::WorldState::Searching => {
                ctx.print_color(0, 0, WHITE, BLACK, "Robot: I want food!")
            }
        }
    }
}

/// Utility function for drawing the world
pub fn draw_world(world: &Arc<RwLock<World>>) {
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
