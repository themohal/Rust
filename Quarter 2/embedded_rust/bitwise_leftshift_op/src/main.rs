fn main() {
    let a:i32 = 3;
    let b:i32 = 2;
    println!("Binary of a:{:b}",a);
    
    println!("Binary of a:{:b}",b);

    let result ;
    result = a << b;

    println!("Binaries {:b},{:b}",a,b);
    println!("Bitwise leftshift binary:{:b}",result);

    println!("Bitwise leftshift decimal:{:}",result);
    

}
