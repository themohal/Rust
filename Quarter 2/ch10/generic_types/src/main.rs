use std::cmp::PartialOrd;
#[warn(unused_variables)]
fn main() {
let v = vec!{1,2,3,4,5};
let v1= vec!{9,8,0,1,2};    
let v2= vec!{'a','z','x'};
    //largest(&v1);
    //let value = largest(&v1);
   // println!("Largest Value :{}",largest(&v));
   // println!("Largest Value :{}",largest(&v1));
    println!("Largest Value :{:#?}",Largestgeneric(&v));
    println!("Largest Value :{:#?}",Largestgeneric(&v1));
    println!("Largest Value :{:#?}",Largestgeneric(&v2));
   
    

//let mut largest = v[0];
//for item in v{
//if item > largest{
  //  largest = item;
//}
//}
//println!("Largest is:{}",largest );
//we are doing the same things again 
//let v = vec!{3,6,10,2,1};
//let mut largest = v[0];
//for item in v{
//if item > largest{
  //  largest = item;
//}
//}
//println!("Largest is:{}",largest );
//so lets make a function
//}
//fn largest(x:&[i32])->i32{ //vector on gives reference 
  //  let mut largest = x[0];
    //for &item in x.iter(){    //we have to use reference and iter iterate vector
    //if item > largest{
      //  largest = item;
    //}
//}
//println!("Largest is:{}",largest );
//lets make function modular return the value 
//largest
//}
    }
fn Largestgeneric<T:PartialOrd+Copy>(x:&[T])->T{ //PartialOrd trait is used when needed for logics and copy trait tells that apply to those which can be copid
    let mut largest = x[0];
    for &item in x.iter(){    //we have to use reference and iter iterate vector
      if item  > largest{
        largest = item;
    }                           //we will need to implement trait partial order
}
largest
}