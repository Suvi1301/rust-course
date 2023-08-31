// This file is a Crate

pub fn printing() {
    println!("Hello this is file 1!");
}


// A Module contained in this crate.
mod maths {
    // submdodule
    pub mod basic_math {
        pub fn multiply(num1: &i32, num2: &i32) -> i32 {
            num1 * num2
        }
    }
}

pub fn rect_area(length: &i32, width: &i32) -> i32 {
    use maths::basic_math::multiply;
    multiply(length, width)
}

/*
file_1 
    - maths
        - basic_math
            - multiplication()
    - rect_area()
*/