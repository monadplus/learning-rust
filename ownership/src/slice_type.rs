pub fn main() {
    //let mut s = String::from("Hello world");
    let s = String::from("Hello world");
    let w = first_word(&s[..]);

    let s2 = "Hello world"; // str literal
    let w2 = first_word(s2);

    //s.clear(); // error: mutable reference

    println!("{}", w);
    println!("{}", w2);
}

//fn first_word(s: &String) -> usize {
//let bytes = s.as_bytes();

//for (i, &item) in bytes.iter().enumerate() {
//if item == b' ' {
//return i;
//}
//}

//s.len()
//}

//let s = String::from("hello");

//let slice = &s[0..2];
//let slice = &s[..2];
//
//let s = String::from("hello");

//let len = s.len();

//let slice = &s[3..len];
//let slice = &s[3..];

// fn first_word(s: &String) -> &str
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// let s = "Hello world"; // is &str
// it's a slice pointing to that specific point of the binary.

// let a = [1,2,3,4,5];
// let slice = &a[1..3]; // type &[i32];
