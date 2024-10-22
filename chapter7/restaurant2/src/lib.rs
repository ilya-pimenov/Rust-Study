use std::collections::HashMap;

use rand::Rng;

//use std::io;
//use stf::fmt;

use std::fmt::Result;
use std::io::Result as IoResult;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

//fn function1() -> fmt::Result {
//    //
//}
//
//fn function2() -> io::Result<()> {
//    //
//}

fn function1() -> Result {}

fn runction2() -> IoResult<()> {}
