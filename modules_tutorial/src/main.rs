use std::collections::HashSet;

use modules_tutorial; // accessing lib.rs in this package
use array_tool::vec::*;

struct Rectange {
    length: i32,
    width: i32,
}

fn main() {
    println!("Hello, world!");
    modules_tutorial::file_1::printing();
    modules_tutorial::file_2::printing();

    let rectangle: Rectange = Rectange { length: 4, width: 2 };
    println!("Area of rectange = {}", modules_tutorial::file_1::rect_area(&rectangle.length, &rectangle.width));

    let vec_1: Vec<i32> = vec![1, 2, 3, 4, 4, 1];
    let vec_2: Vec<i32> = vec![3, 1, 2, 4, 5];

    let intersection: Vec<i32> = vec_1.intersect(vec_2.clone());
    println!("Intersection = {:?}", intersection);
    let union_set: Vec<i32> = vec_1.union(vec_2);
    println!("Union = {:?}", union_set);

    println!("Vec 1 three times displayed = {:?}", vec_1.times(3));

}


/*
RUST MODULE SYSTEM

Package -contains-> Crates -contains-> Modules

Crates can be binary or library crates.
Binary crates can be executed
Library crates can be executed
We can only have 0 or 1 library crate i.e. lib.rs
*/

