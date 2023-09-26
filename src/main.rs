// mod my_funcs;
// mod other_funcs;
// use crate::my_funcs::{add_five, subtract_one};
// use crate::other_funcs::add_10::add_10;

const OUR_COURSE: &str = "Rust with AutoGTP";
fn main() {
    println!("Welcome to this course on {}!", OUR_COURSE);

    //Stack memory
    let x: i32;
    x = 2;
    println!("x is: {}", x);

    let y: i32;
    y = 4;
    println!("y is: {}", y);

    //for loop
    let mut name: &str = "Magnus";
    dbg!(name);
    name = "Magnus1";
    dbg!(name);

    // Map
    let my_emojis: [char; 10] = ['H'; 10];

    let my_arr: [f64; 10] = [0.0; 10];

    let iter_result: String = my_emojis.iter().collect();

    let map_resut = my_arr.map(|n: f64| n + 2.0);

    dbg!(iter_result, map_resut);

    let _dynamic_name: String = String::from("Magnus Sagerup");

    //Closures
    let num: i32 = 5;
    let add_num = |x: i32| x + num;
    let new_num: i32 = add_num(5);
    dbg!(new_num);
}
