use std::{env, rc::Rc};

use bot_lib::{RobotMovement, World, WorldState, WorldUpdate};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

async fn tcp_server() -> anyhow::Result<()> {
    // Allow passing an address to listen on as the first argument of this
    // program, but otherwise we'll just set up our TCP listener on
    // 127.0.0.1:8080 for connections.
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Next up we create a TCP listener which will listen for incoming
    // connections. This TCP listener is bound to the address we determined
    // above and must be associated with an event loop.
    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    // Asynchronously wait for an inbound socket.
    let (mut socket, _) = listener.accept().await?;

    // Create a World
    let mut world = World::default();

    // This will represent the inbound size of the package
    let mut size = [0; std::mem::size_of::<usize>()];

    // In a loop, read data from the socket and write the data back.
    loop {
        // -- READ START --
        // Read the incoming package size
        socket
            .read_exact(&mut size)
            .await
            .expect("failed to read data from socket");

        // Deserialize it
        let size_of_package = usize::from_le_bytes(size);

        // Read the robot_movement package
        let mut robot_movement = vec![0; size_of_package];
        socket
            .read_exact(&mut robot_movement)
            .await
            .expect("failed to read data from socket");

        // -- READ END --

        // Deserialize using bincode
        let robot_movement: RobotMovement = bincode::deserialize(&robot_movement)?;
        // Move the actual robot
        world.move_robot(robot_movement)?;
        let state = world.world_state();
        let update = WorldUpdate {
            world: world.clone(),
            world_state: state,
        };

        // --- WRITE START ---
        let update = bincode::serialize(&update)?;
        let size_le = usize::to_le_bytes(update.len());
        // First write size..
        socket.write_all(&size_le).await?;
        // ..Then the update
        socket.write_all(&update).await?;
        // -- WRITE END --
    }
}

fn main() {
    let mut world = World::custom((10, 10), (10, 11));
    bot_lib::debug::draw_world(&world);
    world.move_robot(crate::RobotMovement::Left).unwrap();
    bot_lib::debug::draw_world(&world);
    // Use this for debugging the world
    //bot_lib::debug::draw_world(&world);
}
