fn main() {
    let byte_escape = "I'm writing \x52\x75";
    println!("{:?}", byte_escape);

    let byte_string = b"I'm writing \x52\x75";
    println!("{:?}", byte_string);

    let ar: [i32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{:?}", ar);

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    println!("{:?}", v);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 100 {
            break counter * 2
        }
    };

    println!("{:?}", result);
}
