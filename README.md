# Bot Tutorial

This is a simple tutorial for people getting up to speed with async [Rust](https://www.rust-lang.org/) using [tokio](https://tokio.rs/).
The goal is to help a small robot get to his food. 
To do this you will need to use async-rust programming foo to help this robot achieve his goal
The server has been made which is a TCP server, including a small debug GUI so that the world state can be visualized. 
Take a look at `server.rs` for an example.

The `R` in the GUI represents the robot and the `F` is the food.

![debug-gui](robot-gui.png)

## Goal

The goal is to create a client that can help get the robot to it's food. 
The code from the shared `lib.rs` can be used to achieve this. 
The datatypes can be serialized over the wire using [bincode](https://github.com/bincode-org/bincode). 

## Before starting

Take a look at the `tests/async_examples.rs` which shows how the async/await code can be used in Rust.
This is similar to async code in Typescript or Python.
It just scratches the surface especially with regards to borrowing but you can read more about that later.
Maybe read the chapter from the book on error handling: [Error Handling](https://doc.rust-lang.org/stable/book/ch09-00-error-handling.html).
We are using [anyhow](https://docs.rs/anyhow/latest/anyhow/) for easy error handling.


## Assignments 

1. The first goal is to familiarize yourself with the `lib.rs` code. To do this you need to complete a test that was written there that checks one of the constrainst from the world.
2. The second goal is to create a client that navigate the robot to the food. 
To do this you need to create a TCP client to connect to the server using tokio. 
An example can be found: [hello-world](https://github.com/tokio-rs/tokio/blob/master/examples/hello_world.rs).
Check the `Cargo.toml` file to check where the second binary is defined it can be run with `cargo r --bin bot_client`


## Open questions

1. There is a potential flaw in the code: for the synchronization primitives I'm using an `std::sync::RwLock` from the standard library. 
Tokio also provides the same primitives but instead the blocking methods can be `.await`'ed. 
Do you have an idea what problem this can cause and why?

