
fn main() {
    let receive= &String::from(dessert());
    println!("{}",receive);

}
fn dessert()-> &String {
    let d1= String::from("Molten Lava");
    &d1
}
