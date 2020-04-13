#[derive(Debug)]
struct Point<T,U>{
    x:T,
    y:U,
}
impl <T,U> Point<T,U>{
    fn mixup <V,W>(self,other:Point<V,W>)->Point<T,W> {
        Point{
            x:self.x,
            y:other.y,
        }
    }
}
fn main() {
    let p1 =Point{x:33,y:5.2};
    let p2 = Point{x:"Hello",y:'a'};
    let p3 = p1.mixup(p2);
    println!("{:#?}", p3);
}
