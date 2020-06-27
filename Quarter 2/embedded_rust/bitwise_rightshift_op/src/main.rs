fn main() {
    let a:i8 = 3;
    let b:i8 = 2;
    println!("Binary of a:{:b}",a);
    
    println!("Binary of a:{:b}",b);

    let result:i8 ;
    result = a >> b; //means shift a to the right 2 times.  

    println!("Binaries {:b},{:b}",a,b);
    println!("Bitwise leftshift binary:{:b}",result);

    println!("Bitwise leftshift decimal:{:}",result);
    

}
