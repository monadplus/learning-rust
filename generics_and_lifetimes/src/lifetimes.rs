// Borrow checker: whether all borrows are valid.
//
// pub fn main() {
//     let r;
//
//     {
//         let x = 5;
//         r = &x;
//     }
//
//     println!("r: {}", r);
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//fn longest<'a>(x: &str, y: &str) -> &'a str {
//let result = String::from("really long string");
//result.as_str()
//}

struct ImportantExcerpt<'a> {
    part: &'a str, // expects a lifetime
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

pub fn main() {
    //let string1 = String::from("abcd");
    //let string2 = "xyz";

    //let result = longest(string1.as_str(), string2);
    //println!("The longest string is {}", result);

    // OK
    //let string1 = String::from("long string is long");

    //{
    //let string2 = String::from("xyz");
    //let result = longest(string1.as_str(), string2.as_str());
    //println!("The longest string is {}", result);
    //}

    // KO
    //let string1 = String::from("long string is long");
    //let result;
    //{
    //let string2 = String::from("xyz");
    //result = longest(string1.as_str(), string2.as_str());
    //}
    //println!("The longest string is {}", result);

    ///////////////////////////////////

    // Struct's lifetimes

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    ////////////////////////////////////

    // Lifetime Elision

    // Compiler can deduce: fn first_word<'a>(s: &'a str) -> &'a str {
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // Lifetime Elision Rules

    //The first rule is that each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

    //The second rule is if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

    //The third rule is if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

    /////////////////////////////////////

    // The Static Lifetime
    //
    // this reference can live for the entire duration of the program.
    //
    // Be careful, you don't usually need that long lifetime.
    let s: &'static str = "I have a static lifetime.";

    /////////////////////////////////

    // Generic Type Parameters, Trait Bounds, and Lifetimes Together
    //
    // All together :3
    use std::fmt::Display;

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
}
