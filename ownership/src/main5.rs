fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    // error
    some_string.push_str(", world");
}
