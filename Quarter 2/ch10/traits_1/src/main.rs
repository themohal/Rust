#[derive(Debug)]
struct Superman{
    name:String,
}
struct Spiderman{
    name:String,
}
struct Hulk{
    name:String,
}
struct Batman{
    name:String,
}
pub trait Power{
    fn power_score (&self)->u8{
        50 //due to duplication of power so put it in default trait
    }

}
impl Power for Superman{
    fn power_score(&self)->u8{ //kind of like method overriding
        100
    }
}
impl Power for Batman{
    fn power_score(&self)->u8{
        80
    }
}
impl Power for Hulk{
  //  fn power_score(&self)->u8{
       // 50
  //}
}
impl Power for Spiderman{
  //  fn power_score(&self)->u8{
   //     50
    //}
} //commented we dont need  them in hulk and spiderman so let them blank already default trait
fn main() {
let my_superman = Superman {
    name:String::from("Superman"),
};
let my_spiderman = Spiderman {
    name:String::from("Spiderman"),
};
let my_hulk = Hulk {
    name:String::from("Hulk"),
};
let my_batman = Batman {
    name:String::from("Batman"),
};

println!("{:#?}",my_superman.power_score() );
println!("{:#?}",my_spiderman.power_score() );
println!("{:#?}",my_hulk.power_score() );
println!("{:#?}",my_batman.power_score() );
}
