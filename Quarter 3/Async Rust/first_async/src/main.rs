// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;
async fn hello_world() {
    println!("hello, world!");
}
async fn learn_song(){
    println!("Learning Started...");
    let mut counter:i64 = 0;
    for i in 1..200000000{
        counter = counter+i;
    }
    println!("Learn Song");
}
async fn sing_song(/*f:impl futures::Future*/){
    println!("Singing Started...");
    //learn_song().await; //this destroy whole concept of async for this we use another function name async main
    let mut counter:i64 = 0;
    for i in 1..100000000{
        counter = counter+i;
    }
    println!("Sing Song");
}
async fn learn_sing_song(){
    learn_song().await;
    sing_song().await;

}
async fn dance(){
    println!("Dance");
}
async fn async_main() {
    let f1 = learn_sing_song();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2); //sequence matter for excution
}
fn main() {
    let future = hello_world(); // Nothing is printed
    
    block_on(future); // `future` is run and "hello, world!" is printed

    //block_on(learn_song());
    //block_on(sing_song());
    //block_on(dance());
    block_on(async_main());//to avoid block on
}
