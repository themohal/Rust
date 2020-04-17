fn main() {
let r;
        {
            let x =10;
            r =&x;
        }
        println!("{}",r ); //r garbage value because x went out of scope 
}
