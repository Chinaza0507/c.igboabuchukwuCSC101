use std::io;
fn main() {
//menu and prices
let _food_menu:[(&str, &str, f32); 5] = [
      ("p","Pounded Yam/Edinkaiko Soup",3200.0),     
      ("F","Fried rice & chicken",3000.0),
      ("A","Amala & Ewedu Soup", 2500.0),
      ("E","Eba & Egusi Soup",2000.0),
      ("W","White Rice & Stew",2500.0),
];

println!("Enter your choice of food_menu");
println!("P = Pounded Yam/Edinkaiko Soup-3200.0"); 
println!("F = Fried rice & chicken-3000.0");
println!("A = Amala & Ewedu Soup-2500.0");
println!("E = Eba & Egusi Soup-2000.0");
 println!("W = White Rice & Stew-2500.0");


 println!("Enter food_menu");

 let mut input = String::new();
 io::stdin().read_line(&mut input).expect("failed to read line");
 let food_menu:f32 = input.trim().parse().expect("Please enter a valid input");
 

 println!("Enter quantity_desired");
 let mut input = String::new();
 io::stdin().read_line(&mut input).expect("failed to read line");
 let quantity_desired:u32 = input.trim().parse().expect("Please enter a quantity within range");

 
 let mut total_cost = food_menu* quantity_desired as f32;
 
 println!("{} * {} = {}",food_menu,quantity_desired,total_cost);

 if total_cost>10000.0 {
     let discount = total_cost * 0.05;
     total_cost -= discount;
     println!("\nYou have received a discount of 5% at {}",discount);
 }else {
     println!("\nYour total_cost is {}",total_cost);
 }


 }
