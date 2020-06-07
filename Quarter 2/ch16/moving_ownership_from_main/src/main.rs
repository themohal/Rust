use std::thread;
fn main() {
    let v = vec![1,2,3,4];
    // let handle =thread::spawn(||{
    // println!("{:?}",v);
    // });
    //     handle.join().unwrap();
    //this will produce error because the variable maynot live until thread ends
    //to tackle this we will need to transfer ownership by using move keyword with closure function
    let handle =thread::spawn(move||{
            println!("{:?}",&v);
            });
        
            handle.join().unwrap();
            
}
