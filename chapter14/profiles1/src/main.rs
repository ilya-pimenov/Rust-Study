use profiles1::mix;
use profiles1::PrimaryColor;

//use profiles1::kinds::PrimaryColor;
//use profiles1::utils::mix;

fn main() {
    println!("Hello, world!");

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
