# Bot tutorial

This is a simple tutorial for people getting up to speed with async rust using tokio.
The goal is to help a small robot get to his food. To do this you will need to use async-rust programming foo to help this robot achieve his goal
The server has been made which is a TCP server, including a small debug GUI so that the world state can be visualized. Take a look at `server.rs` for an example.

## Goal

The goal is to create a client that can help get the robot to it's food. The code from the shared `lib.rs` can be used to achieve this. The datatypes can be serialized
over the wire using bincode. 

## Assignments 

1. The first goal is to familiarize yourself with the `lib.rs` code. To do this you need to complete a test that was written there that checks one of the constrainst from the world.
2. The second goal is to create a client that navigate the robot to the food. To do this you need to create a TCP client to connect to the server using tokio. An example can be found: https://github.com/tokio-rs/tokio/blob/master/examples/hello_world.rs

