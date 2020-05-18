
fn main() {
    println!("Hello, world!");

    let add_one_v2 =|x:u32|->u32{x+1};
    let add_one_v3=|x|{x+1};
    let add_one_v4 =|x| x+1;
    
    println!("{}",add_one_v1(2));
    println!("{}",add_one_v2(3));
    println!("{}",add_one_v3(4));
    println!("{}",add_one_v4(5));
    
}

fn add_one_v1(x:u32)->u32{
    x+1
}

