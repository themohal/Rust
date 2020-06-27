fn main() {
    let mut num = 52;

    println!("number is :{}",num);

    //raw pointer
    let rp1 = &mut num as *mut i32; //creating pointer

    println!("Pointer address to number is: {:?}",rp1);
    //println!("Pointer to number is: {:?}",*rp1);//this will give error because defreferencing is not allowed.
    unsafe{
        println!("Value of Number using pointer : {:?}",*rp1);
    }
}
