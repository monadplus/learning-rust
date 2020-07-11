fn main() {
    let cond = true;

    let number = if cond { 3 } else { 4 };

    if number < 5 {
        println!("condition was true");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else {
        println!("condition was false");
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    while counter != 0 {
        counter -= 1;
    }

    println!("The result is {}", result);

    let a = [1, 2, 3, 4, 5];

    for e in a.iter() {
        println!("The value is: {}", e);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
