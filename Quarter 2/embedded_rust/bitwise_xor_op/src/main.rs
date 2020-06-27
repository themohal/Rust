fn main() {
    let a:i32 = 2;
    let b:i32 = 3;
    println!{"Binary of stored number in a: {:b}",a};
    println!{"Binary of stored number in a: {:b}",b};

    let result :i32;
    result = a ^ b;

    println!{"Bitwise OR | => {:}",result};

}
