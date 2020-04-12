fn main() {
let v = vec!{1,2,3,4,5};
let mut largest = v[0];
for item in v{
if item > largest{
    largest = item;
}
}
println!("Largest is:{}",largest );
}
