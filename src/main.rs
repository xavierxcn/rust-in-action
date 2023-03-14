use std::sync::Arc;


fn main() {
    let mut s1 = String::from("hello");

    let s2 = &s1;


    println!("{s2}");

    s1.push_str(" world");
    println!("{s1}");


}
