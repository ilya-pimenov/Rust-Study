use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

//impl HelloMacro for Pancakes {
//    fn hello_word() {
//        println!("Hello, Macro! My name is Pancakes!");
//    }
//}

// For reference —
//#[route(GET, "/")]
//fn index() {}

pub fn main() {
    let v: Vec<u32> = vec![1, 2, 3];

    println!("hello");

    //let p = Pancakes {};
    //p.hello_macro();

    Pancakes::hello_macro();

    //

    //let sql = sql!(SELECT * FROM posts WHERE id=1);
}
