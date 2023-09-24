mod my_funcs;
mod other_funcs;

use crate::my_funcs::{add_five, subtract_one};
use crate::other_funcs::add_10::add_10;

fn main() {
    let mut x: u32 = 50;
    println!("x is {}", x);

    let number_added_5: u32 = add_five(x);

    println!("x is {}", number_added_5);
    x = 85;
    x = subtract_one(x);

    let z: u32 = add_10(10);
    println!("z is: {}", z);
    println!("x is: {}", x);
}
