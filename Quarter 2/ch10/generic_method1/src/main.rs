#[derive(Debug)]
struct Point <T>{
    x:T,
    y:T,
}
impl <T> Point <T>{ //we need to put <T> otherwise it will not know what T is somewhat like casting
        fn x(&self)->&T{
            &self.x
        }
}
impl Point <f64> {
    fn distance(&self)->f64{
        (&self.x.powi(2)+&self.y.powi(2)).sqrt()
    }
}
fn main() {
    let p1 =Point{x:12,y:33};
    let p2 = Point{x:10.0,y:20.0};
    println!("{:#?}",p1.x());
    println!("{:#?}",p2.distance());
}
