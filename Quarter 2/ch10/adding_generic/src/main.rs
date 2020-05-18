
use std::ops::Add;
use std::fmt::Display;
use std::convert::From;

    fn generic<T: Add+Display>(a:T,b:T)->T::Output{
    a+b 
    }
fn main() {
    let a=3.2;
    let b=4;
    let c =a+b as f64;
        println!("Sum: {:?}",generic(2,3));
}
