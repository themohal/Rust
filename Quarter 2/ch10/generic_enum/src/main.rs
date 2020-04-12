#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}
enum Result<T,U>{
    Ok(T),
    Err(U),
}
fn main() {
    println!("Hello, world!");
}
