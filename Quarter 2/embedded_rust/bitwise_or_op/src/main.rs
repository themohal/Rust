fn main() {
    let a:i32 = 5;
    let b:i32 = 10;
    println!{"Binary of stored number in a: {:b}",a};
    println!{"Binary of stored number in a: {:b}",b};

    let result :i32;
    result = a | b;

    println!{"Bitwise OR | => {:}",result};

}
