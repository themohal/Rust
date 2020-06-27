fn main() {
    let a : i32 = 10;
    let b : i32 = 15;
    println!("Binary of stored number in a : {:b}",a);
    println!("Binary of stored number in b : {:b}",b);
    let result:i32;

    result = a & b; //applies and operation in binary 
    println!("Bitwise AND : {}",result);
}
