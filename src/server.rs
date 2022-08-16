use bot_lib::World;

fn main() {
    let world = World::default();
    bot_lib::debug::draw_world(&world);
}
