//Main function starts from here
fn main() 
{
//variable declaration and intialization
    let value1 = 30;
    let value2 = 12;
//declaring variables and then intializing the results into them.
    let sum = value1+value2;    //sum
    let div:f32 = value1 as f32/value2 as f32; //divison and casting to get the floating points numbers
    let mul = value1*value2; //multiplying
    let sub = value1-value2; //subtracting
    let modulus = value1 % value2 ; //modulus  
    //printing the values of arithmetic operations on screen
        println!("Sum is :{}",sum); 
        println!("Div is :{}",div );
        println!("Mul is :{}",mul);
        println!("Sub is :{}",sub);
        println!("Mod is :{}",modulus);
}
//End of Main