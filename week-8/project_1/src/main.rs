use std::io::Write;
use std::fs::File;

fn main() {
    // Define the categories and brands
    let lager = vec!["33 export", "Desperdos", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williams"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    // To overwrite a file
    let mut file = File::create("drinks.txt").expect("create failed");

    // Write the data to the file 
    file.write_all("Lager:\n".as_bytes()).unwrap();
    file.write_all(lager.join(", ").as_bytes()).unwrap();
    file.write_all("\n\nStout:\n".as_bytes()).unwrap();
    file.write_all(stout.join(", ").as_bytes()).unwrap();
    file.write_all("\n\nNon-Alcoholic:\n".as_bytes()).unwrap();
    file.write_all(non_alcoholic.join(", ").as_bytes()).unwrap();

    println!("Data saved to drinks.txt!");
}