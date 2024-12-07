use std::io;
fn main() {
    println!("Welcome to EY job search requirements");
     let mut input = String::new();
     println!("How many programmers do you want to enter");
     io::stdin().read_line(&mut input).expect("failed to read input");
     let n:u32 =input.trim().parse().expect("failed to parse input");

     let mut programmer_vector:Vec<(String, f32)>= Vec::new();
      for x in 0..n {
        let mut input1 = String::new();
        
        println!("Enter name");
         io::stdin().read_line(&mut input1).expect("failed to read input");
     let name:String =input1.trim().parse().expect("failed to parse input");

     let mut input2 = String::new();
     println!("Enter your years of experience ");
     io::stdin().read_line(&mut input2).expect("failed to read input");
     let years_of_experience:f32 =input2.trim().parse().expect(" input not valid ");

     let programmer:(String,f32) = (name,years_of_experience);
     programmer_vector.push(programmer);
     println!("Added programmer {}", x);
}
programmer_vector.sort_by(|a, b|b.1.partial_cmp(&a.1).unwrap());
let  experienced_programmer = &programmer_vector[0];
println!("{:?}",experienced_programmer );

}
