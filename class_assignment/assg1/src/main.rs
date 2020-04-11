pub mod weather{
    pub mod temp {
        pub fn print(){
            println!("Its too cold !!!");
        }
    }
}
use weather::temp::print;
fn main() {
    weather::temp::print();
    println!("Hello, world!");
}
