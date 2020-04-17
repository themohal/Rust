fn main() {
    let s1 = String::from("Hello World");
    let result;
    {
        let s2 =String::from("Hello Pakistan");
        result = longest(&s1.as_str(),&s2.as_str());
        println!("{}",result );
        
    }
    //s2 drop
   // println!("{}",result ); out of scope s2 not memory safe lifetime changed
}//s1 drop

fn longest<'a>(a:&'a str,b:&'a str)->&'a str{
    if a.len() > b.len(){
        &a
    }else{
        &b
    }
}
