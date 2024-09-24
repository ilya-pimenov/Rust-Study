use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 80 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "     ";
    let spaces = spaces.len();

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let x: (i32, f64, u8) = (500, 6.4, 1);

    //let (x, y, z) = tup; // destructure tuple

    let five_hunderd = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("Tuple {x:?}");

    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [3; 5];

    let first = a[0];
    let second = a[1];

    //
    //
    //
    //

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
