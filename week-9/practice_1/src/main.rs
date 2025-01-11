fn main() {
    let v = vec![101,250,330,400];
    //vector v owns the object in the heap

    //only a single variable owns the heap memory at any given time
    let v2 =&v;
    //here two variables owns the heap value
    //two poinmters to the same content is not allowed in rust

    //Rust is very smart in terms of memory access, so it detects a race condition
    //as tow variabe point to same heap

    println!("{:?}",v);
}
