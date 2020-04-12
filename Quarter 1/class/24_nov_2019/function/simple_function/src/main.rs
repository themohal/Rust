fn main() {
    hello();
    loop_passing(4,100);
    println!("We got the value: {}",retu_func() );
    let ret_value = retu_func();
    println!("Returned value in varibale:{}",ret_value );
    let val = sendreceive(15);
    println!("Value returned :{}",val );
}
fn hello(){
    println!("Hello, world!");
}
//passing value
fn loop_passing(index:i32,price : u16){
    let mut data = 0;
    while data <= index {
        println!("Index :{}",data);
        data =data+1;
    }
    println!("Price is:{}",price );
}
//return value function
fn retu_func() -> u16 { //the -> shows this functions returns something datatype is must
    println!("We are in return function" );
    1472
}
fn sendreceive(data :u16) -> u16{
    println!("We are in send receive function" );
 data*data 
}
