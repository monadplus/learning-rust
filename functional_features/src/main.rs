use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    // closure
    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        // |param1, param2 ...|
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    // Capturing the Environment with Closures
    //
    // Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter:
    //  - taking ownership
    //  - borrowing mutably
    //  - and borrowing immutably.
    //
    //  These are encoded in the three Fn traits as follows:
    //
    //  - FnOnce: consumes the variables it captures from its enclosing scope, known as the closure’s environment. To consume the captured variables, the closure must take ownership of these variables and move them into the closure when it is defined. The Once part of the name represents the fact that the closure can’t take ownership of the same variables more than once, so it can be called only once.
    //  - FnMut: can change the environment because it mutably borrows values.
    //  - Fn: borrows values from the environment immutably.
    //
    //  When you create a closure, Rust infers which trait to use based on how the closure uses the values from the environment.

    let x = 4;
    let equal_to_x = |z| z == x; // with fn this doesn't work.

    // move: take ownership in the closure (even if it is not necessary)
    let equal_to_x_ownership = move |z| z == x;
    //println!("can't use x here: {:?}", x);

    assert!(equal_to_x(4));
    assert!(equal_to_x_ownership(4));
}

// Be careful with Cacher
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
