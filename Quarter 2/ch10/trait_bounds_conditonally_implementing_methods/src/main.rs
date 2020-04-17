use std::collections::HashMap;
use std::fmt::Display;
#[derive(Debug)]
struct Pair<T> {
    x:T,
    y:T,
}
impl <T> Pair<T>{
    fn new(x:T,y:T)->Self{ //Self is an alias for our datatype this function will create instance
      Self{  x,
        y,
      }
    }

}
pub trait Summary{
    fn summarize(&self)->String;
}
impl <T:Display+PartialOrd> Pair <T> {
        fn check(&self){
            if self.x>self.y {
                println!("X: {} is greater then Y: {}",self.x,self.y);
            }else{
                println!("Y: {} is greater then X: {}",self.y,self.x);
            }
        }
}

fn main() {
    let mut h = HashMap::new(); //we can implement because no trait is implemented for hashmap
    h.insert(4,3);
    let mut h2= HashMap::new();
    h2.insert(6,7);

    //let p1 = Pair::new(1,4);
    let p1 = Pair::new(h,h2);
    println!("{:#?}",p1 );
    //p1.check();//we cannot do this for hashmap because it was defined that both display and partialord trait should be implemented
    
}

