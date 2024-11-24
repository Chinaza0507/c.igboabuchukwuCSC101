use std::io;
fn main() {
//menu and prices
let mut total_cost:f32 = 0.0;
loop {


println!("Enter your choice of food_menu");
println!("P = Pounded Yam/Edinkaiko Soup-3200.0"); 
println!("F = Fried rice & chicken-3000.0");
println!("A = Amala & Ewedu Soup-2500.0");
println!("E = Eba & Egusi Soup-2000.0");
 println!("W = White Rice & Stew-2500.0");


  
 println!("Enter food_menu");

 let mut input = String::new();
 io::stdin().read_line(&mut input).expect("failed to read line");
 let food_menu = input.trim();
if  food_menu == "QUIT" {
    break;
}

 println!("Enter quantity_desired");
 let mut input = String::new();
 io::stdin().read_line(&mut input).expect("failed to read line");
 let quantity_desired:f32 = input.trim().parse().expect("Please enter a quantity within range");
let   mut price:f32 = 0.0;
if food_menu == "P" {
     price = 3200.0;
}else if food_menu == "F" {
      price = 3000.0;
}else if  food_menu == "A" {
      price = 2500.0;
}else if  food_menu == "E" {
     price = 2000.0;
}else if  food_menu == "W" {
      price = 2500.0;
}
 
 let cost =  price * quantity_desired ;
 total_cost += cost;
       
 
 println!("Added {} to your total. current total: {}",cost ,total_cost);
  }  
 if total_cost>10000.0 {
     let discount = total_cost * 0.05;
     total_cost -= discount;
     println!("\nYou have received a discount of 5% at {}",discount);
 }else {
     println!("\nYour total_cost is {}",total_cost);
 }


 }

