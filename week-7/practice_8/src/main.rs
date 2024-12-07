fn main() {
    //initialize a mutable tuple
    let mut mountain_heights = ("Everest", 8848, "Fishtail", 66933);

    println!("Original tupule = {:?}", mountain_heights);
    //CHANGE 3RD AND 4TH ELEMENT OF A MUTABLE TUPLE
    mountain_heights.2 ="Lhotse";
    mountain_heights.3 = 8516;

    println!(" Changed tuple = {:?}", mountain_heights);
}
