pub fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string(); // Display trait
    let s = "initial contents".to_string();
    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");

    // The + Operator

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // +Operator ~ fn add(self, s: &str) -> String
    //
    // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str. When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //println!("{}", s1); // s1 value borrowed on add() call.

    // The format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    // Indexing Strings: not allowed (see
    // https://doc.rust-lang.org/stable/book/ch08-02-strings.html#internal-representation)

    let s1 = String::from("hello");
    //let h = s1[0]; // error

    // A String is a wrapper over a Vec<u8>. Let’s look at some of our properly encoded UTF-8

    // Slicing Strings: not a good idea (not clear the return type).

    let hello = "Здравствуйте";

    let s = &hello[0..4]; // first 4 bytes of the String: Зд
                          // &hello[0..1]; // panic

    // Iterating

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
