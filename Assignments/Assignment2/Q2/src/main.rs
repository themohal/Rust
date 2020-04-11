//Main function starts from here
fn main() 
{
//tuple declaration and intialization
    let fruit = ("Mango",5,500);
    //Destructring tuple
    let (name,weight,price) = fruit;
    //Destrcutring can also be done like below
    //let name= fruit.0;
    //let weight= fruit.1;
    //let price = fruit.2;
    //printing the tuple on screen
    println!("Name of Fruit :{}",name);
    println!("Weight of Fruit in Kgs :{}",weight);
    println!("Price of Fruit :{}",price);
}
//End of Main