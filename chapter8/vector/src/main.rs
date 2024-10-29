fn main() {
    println!("Hello, world!");

    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    //

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Immutable borrow and mutable in the same context, won't work

    //let mut v = vec![1, 2, 3, 4, 5];

    //let first = &v[0];

    //v.push(6);

    //println!("The first element is: {first}");

    //

    let v = vec![100, 32, 58];
    for i in &v {
        println!("{i}");
    }

    //

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("Updated vector is {v:?}");

    // Holding different types in an enum
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

    // Vector is freed when it goes out of scope
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes of scope and is freed here
}
