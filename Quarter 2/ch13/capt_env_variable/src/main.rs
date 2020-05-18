fn main() {
    //closure can access any varibale of a scope where it is defined
    let x =4;
    
    let equal_to_x =|z:i32|z==x;
//assert takes the value and compares to the other value if both are same then
//program runs succesfully if not then panic error
    //assert!(equal_to_x(4));
    println!{"{:?}",assert!(equal_to_x(4))};
    println!{"{:?}",assert!(equal_to_x(3))}; //panic error

}
