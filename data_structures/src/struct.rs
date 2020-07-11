#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Multiple impl blocks is allowed.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // associated functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 45,
    };

    let square = Rectangle::square(5);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("ret1 is {:?}", rect1);
    println!("ret1 is {:#?}", rect1);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("Square: {:?}", square);
}
