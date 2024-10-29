fn main() {
    println!("Hello, world!");

    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    //

    let s = String::from("initial contents");

    //

    let mut s = String::from("foo");
    s.push_str("bar");

    //

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    //

    let mut s = String::from("lo");
    s.push('l');

    //

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    //

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    //

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    //

    let s1 = String::from("hello");
    let h = s1[0];

    // Won't compile as З stands for two bytes 208 151 and you don't expect a byte back
    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    //

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    // Iterating through a string with chars and bytes

    for c in "Зд".chars() {
        println!("{c}");
    }

    //

    for b in "Здравствуйте".bytes() {
        println!("{b}");
    }
}
