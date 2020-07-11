struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple structs
struct Color(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("another@example.com");

    let user2 = build_user(String::from("arnau@foo.com"), String::from("Arnau"));

    let user3 = User {
        email: String::from(""),
        username: String::from(""),
        ..user1
    };

    let black = Color(0, 0, 0);
    black.0;
    black.1;
    black.2;

    //println!("{}", user1);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
