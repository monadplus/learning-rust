// Unwinding the Stack or Aborting in Response to a Panic
//
// see https://doc.rust-lang.org/stable/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic
//

use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // >>> RUST_BRACKTRACE=1 cargo run
    //
    // panic!("die");
    //
    // let v = vec![1, 2, 3];
    // v[99];

    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // std::result::Result<std::fs::File, std::io::Error>
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("Failed to open hello.txt"); // panic error message

    match read_username_from_file() {
        Ok(name) => println!("Name is {}", name),
        Err(err) => panic!("{}", err),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// the ? Operator
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
// ^^^^^ equivalent to fs::read_to_string("hello.txt");

use std::error::Error;

fn main_2() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
