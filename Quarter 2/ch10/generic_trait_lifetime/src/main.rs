use std::fmt::Display;
fn main() {
println!("{}",longest_with_announcement("hello","Hi","Farjad"));
        
}

fn longest_with_announcement<'a,T:Display>(x:&'a str,y:&'a str,ann:T)->&'a str{
    println!("{}",ann );
    if x.len()> y.len(){
        x
    }else{
        y
    }
}