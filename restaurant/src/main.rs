use std::collections::HashMap;

pub fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

//     use std::fmt;
//     use std::io;
//
//     or
//
//     use std::fmt::Result;
//     use std::io::Result as IoResult;
//
//     fn function1() -> fmt::Result {
//         // --snip--
//     }
//
//     fn function2() -> io::Result<()> {
//         // --snip--
//     }

// front_of_house.rs
mod front_of_house;

// Re-exporting:
//
// By using pub use, external code can now call the add_to_waitlist function using hosting::add_to_waitlist
pub use crate::front_of_house::hosting;

use std::{cmp::Ordering, io};

use std::collections::*;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
