fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    // But mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope.
    let r1 = &mut s;
    let r2 = &mut s;
    println!("{}, {}", r1, r2);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//  Users of an immutable reference don’t expect the values to suddenly change out from under them!
//
//    let mut s = String::from("hello");
//
//    let r1 = &s; // no problem
//    let r2 = &s; // no problem
//    let r3 = &mut s; // BIG PROBLEM
//
//    println!("{}, {}, and {}", r1, r2, r3);

// Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used
//
//    let mut s = String::from("hello");
//
//    let r1 = &s; // no problem
//    let r2 = &s; // no problem
//    println!("{} and {}", r1, r2);
//    // r1 and r2 are no longer used after this point
//
//    let r3 = &mut s; // no problem
//    println!("{}", r3);
