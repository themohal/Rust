#[derive(Debug)]
struct Food {
    name:String,
    price : u16,
    serving : u8,
}
impl Food {
    fn ret_str(name:String,price:u16,serving:u8) -> Food { //associated function
        Food{
            name,
            price,
            serving,
        }
    }    
}

fn main() {
let name = "Bread & Omelete".to_string();
let price = 450;
let serving = 2;
   
    let nashta1 = ret_str(name,price,serving);
    //println!("{:#?}",nashta1);
   let nashta2 = Food :: ret_str(name,price,serving);
    println!("{:#?}",nashta2);

    }

fn ret_str(name:String,price:u16,serving:u8) -> Food {
        Food{
           name,
            price,
            serving,
        }
   }