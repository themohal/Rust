use std::thread;
use std::time::Duration;
fn main() {
    // //single threaded code
    // for i in 1..10{
    //     println!("Executing from i looop:{}",i);
    // }
    // for j in 1..10{
    //     println!("Executing from j loop:{}",j);
    // }

    //to run both loops simultenosly we will need to use multithreads
    //first we will need to include two libs time and thread
    //thread::spawn(|| {}); //this function takes closure function as input
    thread::spawn(||{
        for i in 1..10{
            println!("Executing from new thread:{}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });    
    
        for j in 1..10{
                println!("Executing from main thread loop:{}",j);
                thread::sleep(Duration::from_millis(1));
            }
}
