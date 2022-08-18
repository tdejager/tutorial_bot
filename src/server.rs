use std::{
    env,
    sync::{Arc, RwLock},
};

use bot_lib::{RobotMovement, World, WorldUpdate};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

async fn tcp_server(world: Arc<RwLock<World>>) -> anyhow::Result<()> {
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

        // Deserialize it, note that we are using little-endian is a representation
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
        world.write().unwrap().move_robot(robot_movement)?;
        let state = world.read().unwrap().world_state();
        let update = WorldUpdate {
            world: world.read().unwrap().clone(),
            world_state: state,
        };

        // --- WRITE START ---
        let update = bincode::serialize(&update)?;
        //.. again little-endian
        let size_le = usize::to_le_bytes(update.len());
        // First write size..
        socket.write_all(&size_le).await?;
        // ..Then the update
        socket.write_all(&update).await?;
        // -- WRITE END --
    }
}

/// Small demo to demonstrate movement
#[allow(dead_code)]
fn demo(world: Arc<RwLock<World>>) -> tokio::task::JoinHandle<()> {
    // This spawns a seperate tokio task
    // Which is like a thread inside the tokio runtime
    tokio::spawn(async move {
        // Sample of moving the robot
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        world
            .write()
            .unwrap()
            .move_robot(crate::RobotMovement::Left)
            .unwrap();
    })
}

/// Spawn the TCP server inside a seperate
fn spawn_tcp(world: Arc<RwLock<World>>) -> tokio::task::JoinHandle<()> {
    // This spawns a seperate tokio task
    // Which is like a thread inside the tokio runtime
    tokio::spawn(async move {
        tcp_server(world)
            .await
            .expect("error while running tcp server")
    })
}

/// This ensures that we are inside the tokio run-time
#[tokio::main]
async fn main() {
    // This creates a World object inside an atomic reference counted struct inside a Mutex
    // this creates a thread-safe datastructure to be able to update
    // which is needed because we use it in both the GUI and the server potentially
    // read more on this at: https://tokio.rs/tokio/tutorial/shared-state
    let world = Arc::new(RwLock::new(World::default()));
    let world_clone = world.clone();

    // Uncomment this if you want to see a small demo, comment the `spawn_tcp`
    //demo(world_clone);
    spawn_tcp(world_clone);

    // Use this for debugging the world, this does not use async/await code
    // this call is blocking but because it spawns a window it should be done from the
    // main thread
    bot_lib::debug::draw_world(&world);
}
