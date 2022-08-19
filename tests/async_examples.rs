/// This shows some basic async/await examples using tokio
/// Run these tests using `cargo t tutorial -- --nocapture` the `--nocapture` shows the output. The
/// `tutorial` part just runs all the tests prefixed with `tutorial`
/// Normally these files outside the source are for integration tests. See: https://doc.rust-lang.org/book/ch11-03-test-organization.html
/// However see this as a small tutorial
use std::{sync::Arc, time::Duration};
use tokio::{sync::RwLock, time::sleep};

async fn hello() {
    println!("Hello");
    sleep(Duration::from_millis(1)).await;
}

async fn goodbye() {
    println!("Goodbye");
}

async fn multiply_stuff(x: u32, y: u32) -> u32 {
    sleep(Duration::from_millis(3)).await;
    x * y
}

/// The first test with 2 simple await statements
/// the #[tokio::test] attribute makes sure the function is run in the tokio_test
/// runtime
#[tokio::test]
async fn tutorial_basics() {
    // First run this
    hello().await;
    // .. then this
    goodbye().await;
}

/// This function shows that we can use values that come from an await
#[tokio::test]
async fn tutorial_basics2() {
    // First run this
    let calc = multiply_stuff(3, 3).await;
    // .. then this for something
    println!("Use this value {}", calc);
}

/// This shows you can use tokio constructs to run things concurrently
#[tokio::test]
async fn tutorial_concurrent() {
    // The join is run concurrently
    let (x, y) = tokio::join!(multiply_stuff(3, 4), multiply_stuff(5, 6));

    // This section was taken from the docs of tokio::joint and is important:

    // By running all async expressions on the current task, the expressions are
    // able to run **concurrently** but not in **parallel**. This means all
    // expressions are run on the same thread and if one branch blocks the thread,
    // all other expressions will be unable to continue. If parallelism is
    // required, spawn each async expression using [`tokio::spawn`] and pass the
    // join handle to `join!`.
    //

    println!("{} {}", x, y);

    // Often you don't care about parallelism in async/await code because the tasks
    // are suspened when a `.await` is encountered so the scheduler can switch between the tasks
    // this is called coopertive scheduling
}

/// This shows you an example how to run stuff in parallel
/// like the previous stated we will use `spawn` to spawn parallel tasks
#[tokio::test]
async fn tutorial_parallel() {
    // These return JoinHandle<()> we can wait for the task to complete
    // Spawn runs them in parallel when multithreaded runtime is being used
    // which is default for tokio when *not* using tokio::test
    let task_1 = tokio::spawn(async { multiply_stuff(3, 3).await });
    let task_2 = tokio::spawn(async { multiply_stuff(3, 10).await });

    // These return Result types because the join can fail
    let (x, y) = tokio::join!(task_1, task_2);
    println!("{} {}", x.unwrap(), y.unwrap());

    // Note that we could have also run `task_1.await; task_2.await` but this orders how we await
    // the joins, while the join does not have this restriction task_2 can finish before task_1

    // Another primitive that is often used is the tokio::select! this waits only for the first branch to be completed
    // This is useful when working with cancellation see: https://tokio.rs/tokio/tutorial/select for more info
}

#[tokio::test]
/// Final tutorial that teaches you about 'sharing' data between spawned tasks
/// this is kinda the same as in Rust just using threads so you can also read
/// the chapter: https://doc.rust-lang.org/stable/book/ch16-03-shared-state.html
/// or: read more on this at: https://tokio.rs/tokio/tutorial/shared-state (for a tokio take on it)
/// as a reference
async fn tutorial_data_sharing() {
    // We created a reference counted value which lets us have multiple owners
    // we use a ReadWrite lock to synchronize access so that this is safe
    // this is a common pattern when working with multithreaded code
    // You need to be able to copy it over threads somehow in this case the `Arc`
    // but you also need to synchronize access in this case the `RwLock`
    //
    // channels also exist which can be used to send values and is often a bit more
    // flexible than this
    let shared = Arc::new(RwLock::new(0));

    let shared_clone = shared.clone();
    let t1 = tokio::spawn(async move {
        for _ in 0..10 {
            let mut a = shared_clone.write().await;
            *a += 1;
        }
    });

    let shared_clone = shared.clone();
    let t2 = tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(3)).await;
        for _ in 0..10 {
            let mut a = shared_clone.write().await;
            *a += 1;
        }
    });

    t1.await.unwrap();
    // Small excercise: Try removing an t2.await and see what happens
    // why does this happen
    t2.await.unwrap();

    // Should be 20
    assert_eq!(*shared.read().await, 20);
}
