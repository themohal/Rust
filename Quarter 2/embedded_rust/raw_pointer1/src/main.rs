fn main() {
    let mut num = 26;
    //println!("Hello, world!");



    let mut string =  "Hello Unsafe Rust".to_string();

    println!("Value of String: {}",string);
    println!("Pointer of String: {:p}",&string);

    //creating raw pointer
    //let variablename = reference/address as type-of parameter;

    let ref1 = &mut string as *mut String;

    println!("{:?}",ref1);
    
    
    unsafe{
    println!("{:?}",*ref1);
    *ref1 = "".to_string();
    println!("After updating value: {:?}",*ref1);
    drop(ref1);
    }
    
    println!("Value of String: {}",string);
    println!("{:?}",ref1);

}
