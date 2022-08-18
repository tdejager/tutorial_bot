/// This is the executable to write the client in

/// The tokio::main macro just converts the main function into:
///fn main() {
//     let mut rt = tokio::runtime::Runtime::new().unwrap();
//     rt.block_on(async {
//          ...
//     })
// }
// This just runs the main function inside the tokio Runtime
#[tokio::main]
async fn main() {
    println!("I want to feed me bots!")
}
