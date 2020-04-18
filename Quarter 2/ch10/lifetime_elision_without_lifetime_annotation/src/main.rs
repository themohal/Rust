fn main() {
// this program is running without using lifetime annotation
println!("{:#?}",first_word("Hello World") );

}
//this function is returning value without usign lifetime
fn first_word(s:&str /*input liftime parameter*/)->&str/*output lifetime parameter*/{
    let bytes =s.as_bytes();
    println!("{:#?}",bytes );
    for (i,&item) in bytes.iter().enumerate(){
        if item ==b' '{ //ascii of empty space is 32
            return &s[0..i]; //in our case i will iterate to index 5
        }
    }
    &s[..] //this is returning from index 0 till the last index if we remove space then this will be returned conditon willl never get true
}
/*compiler infers liftime of reference automatically 
following 3 lifetime elison rules
1st rule is applied to input lifetime parameter
other 2 rules are applied on output lifetime parameter
these rules are implemented on function bodies and impl block bodies
*/