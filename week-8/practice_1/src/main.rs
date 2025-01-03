use std::io::Write;
fn main() {
    let announce = "Week 8 - Rust File Input & Output\n";
    let dept = "Department of computer science";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to Rust Progrqamming \n"
        .as_bytes()).expect("write failed");
    file.write_all(announce.as_bytes()).expect("write failed");
    file.write_all(dept.as_bytes()).expect("write failed");
    println!("\n Data written to file");
    
}
