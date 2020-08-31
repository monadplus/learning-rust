mod lib;
mod lib2;
mod lib3;
// use lib::*;

// Box<T>

enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // The Box<T> type is a smart pointer because it implements the Deref trait, which allows Box<T> values to be treated like references.
    //
    // Boxes allow you to store data on the heap rather than the stack
    //
    // You’ll use them most often in these situations:
    //
    // - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    //
    // - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    //
    // - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type
    //
    // The third case is known as a trait object, and Chapter 17 devotes an entire section, “Using Trait Objects That Allow for Values of Different Types,” just to that topic

    let b = Box::new(5); // Box<i32>, allocated on the heap.
    println!("b = {}", b);

    // Just like any owned value, when a box goes out of scope, as b does at the end of main, it will be deallocated. The deallocation happens for the box (stored on the stack) and the data it points to (stored on the heap).
    //
    // When a Box<T> value goes out of scope, the heap data that the box is pointing to is cleaned up as well because of the Drop trait implementation

    // **** Enabling Recursive Types with Boxes *****

    // At compile time, Rust needs to know how much space a type takes up.
    //
    // **Recursive type**: type whose size can't be known at compile time.
    //
    // However, boxes have a known size, so by inserting a box in a recursive type definition, you can have recursive types.

    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));

    lib::deref_trait();
    lib::drop_trait();
    lib::rc();
    println!("");
    lib2::ref_cell();
    println!("");
    lib3::leak_memory();
}
