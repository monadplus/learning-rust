#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

// Running Tests in Parallel or Consecutively
//
// When you run multiple tests, by default they run in parallel using threads. This means the tests will finish running faster so you can get feedback quicker on whether or not your code is working.
//
// $ cargo test -- --test-threads=1
// $ cargo test -- --show-output // print output of success tests
// $ cargo test it_works // runs the two tests that starts with it_works*
// $ cargo test -- --ignored // only run ignored tests
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // assert_ne
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works_2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    //#[test]
    //fn another() {
    //panic!("Make this test fail");
    //}

    use super::*; // Rectangle and can_hold()

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        //assert!(smaller.can_hold(&larger)); // panic!
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[ignore]
    fn it_adds_two() {
        // macro requires PartialEq + Debug
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    //#[should_panic]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
