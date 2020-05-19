
fn main() {
    let x = vec![1,2,3,4];
    
    // //let mut mem="";
    // //let mut equal_to_x = move||{
    //    let z="hello";
    //    mem=&z;
    // }; when we used move keyword it didnt print mem and was clear when we used without move then it printed
    //equal_to_x();
    //println!("{:?}",mem);
    //let equal_to_x =move||{let z=x;};
      //  equal_to_x();
       // println!("can't access x here {:?}",x);//error value borrowed here after move

       let equal_to_y =move|z|z==x;
       
    let y = vec![1,2,3,4];
    assert!(equal_to_y(y));

}
