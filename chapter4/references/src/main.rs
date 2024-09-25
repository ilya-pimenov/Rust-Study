fn main() {
    println!("Hello, world!");

    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length if '{s1}' is {len}.");

    //

    let mut s = String::from("hello");

    change(&mut s);

    println!("s is {s}");

    //

    /*
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
    */

    //

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");

    //

    let refernce_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
