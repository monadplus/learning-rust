pub fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new(); // Vec<i32>
    v.push(5);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no thrid element."),
    }

    let v = vec![1, 2, 3, 4, 5];

    //let does_not_exist = &v[100]; //panic
    let does_not_exist = v.get(100);

    // ownership
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0]; // error: cannot borrow `v` as mutable because it is also borrowed as immutable
    // v.push(6);
    // println!("The first element is {}", first);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match row.get(0) {
        Some(SpreadsheetCell::Int(i)) => println!("Value of i {}", i),
        _ => println!("Not expected value."),
    }
}
