fn main() {
    let s1 = "abcd";
    let s2 = "xyz";
    println!("Value :{}",genericlongest(&s1,&s2));
}
/*fn LongesString(x1:& str,x2:& str)->& 'a str{ //&str missing lifetime 
    //this function does not know which value to return either x1 or x2 casuing error
    if x1.len() > x2.len(){
        &x1
    }else{
        &x2
    }

}*/
//lets make function with generic lifetime
fn genericlongest<'a>(x1:&'a str,x2:&'a str)->&'a str{ //'a annotation tells the lifetime
//value being passed as parameter from main and value being returned now has the same lifetime
    if x1.len() > x2.len(){
        &x1
    }else{
        &x2
    }
}
