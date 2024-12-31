use std::fmt;
use std::io::Error;

type Kilometers = i32;

fn main() {
    println!("Hello, world!");

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

//pub trait Write {
//    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
//    fn flush(&mut self) -> Result<(), Error>;
//    
//    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
//    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
//}

//

fn bar() -> ! {
    // --snip--
}

//fn generic<T: ?Sized>(t: &T) {
//    // --snip--
//}

//

type Result<T> = std::result::Result<T, stc::io::Error>

pub trait Write{
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

// Box<dyn Fn() + Send + 'static>
//
// let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
//
// fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
// // --snip--
// }
//
// fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
// // --snip--
// }

type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {
    // --snip--
}

fn returns_long_type(f: Thunk) {
    // --snip--
}
