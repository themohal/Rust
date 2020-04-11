fn main() {

{ //our scope for stringtype
    let name1 = "PIAIC"; // string literal
    let name = String::from("PIAIC");//stringtype
    let name3 = "PIAIC".to_string(); // stringliteraltostringtype
    let stringlateral = "PIAIC"; 
    let name4 = stringlateral.to_string(); //literal to type
    println!("Name 3: {}",name3);
} //heap data is dropped

{//stack
    let age:u8 =23;
    println!("Age :{}",age );

}//stack data is popped

}