fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

pub fn main() {
    let mut s = String::new(); // String::from("Foo");

    add_content(&mut s);

    println!("{}", s);
}

fn add_content(&mut String) {
    s.push_str("Foo");
}
