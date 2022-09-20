#![allow(unused)]

mod lib;
//use alibtry::eat_at_restaurant;  на этот вариант жалуется цвет вс, но все компилится и работает??
//use crate::lib::eat_at_restaurant; это работает вместо добавления перед eat_at_restaurant();

fn main() {
    println!("Hello, world!");
    crate::lib::eat_at_restaurant();
}

/*
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();  избегаем длинного пути и понятно откуда взялось
    map.insert(1, 2);
} 
*/
//reexporting
/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
pub use crate::front_of_house::hosting;   //<- pub use  делает для всех паб 
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
 */
//nested path
/*
|use std::cmp::Ordering;
|use std::io; 
-> |use std::{cmp::Ordering, io};
|use std::io;
|use std::io::Write;
-> |use std::io::{self, Write};
*/
/* vse absolutno import  (std)
#![allow(unused)]
fn main() {
use std::collections::*;
}
 */