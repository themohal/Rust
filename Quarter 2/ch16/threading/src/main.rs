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
    // thread::spawn(||{
    //     for i in 1..10{
    //         println!("Executing from new thread:{}",i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });    
    
    //     for j in 1..5{
    //             println!("Executing from main thread loop:{}",j);
    //             thread::sleep(Duration::from_millis(1));
    //         }
    // //here can occur a problem where new thread taking more time then main thread
    //if we decrease the range of main thread loop it will not let new thread to completly run
    //because the process is killed to solve this problem we use handle.join().unwrap
    
    let handle =thread::spawn(||{
        for i in 1..10{
            println!("Executing from new thread:{}",i);
            thread::sleep(Duration::from_millis(1));
        }
    }); 
    //handle.join().unwrap(); if we use it here then our custom will excute first
    //but not simultenosly with main but both will excute
    
        for j in 1..5{
                println!("Executing from main thread loop:{}",j);
                thread::sleep(Duration::from_millis(1));
            }
        &handle.join().unwrap();//it is necessary to call it at the end so that all
        //threads could run simultenosly
}
