fn main() {
    let v = vec![20,40,60,80];
    //vecctor v owns the object in heap

    let v2 = v;

    let v2_return = display(v2);
    println!(" In main{:?}",v2_return);
}
 fn display(v:Vec<i32>)->Vec<i32>{
    //RETURNING SAME VECTOR
    println!("Iside display {:?}",v);
    return v;
 }
