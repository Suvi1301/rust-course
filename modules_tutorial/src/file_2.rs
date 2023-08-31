// This file is a Crate

use self::person::personal_info;

pub fn printing() {
    println!("Hello this is file 2!");
}

mod person {
    pub struct personal_info {
        age: u8,
        pub education: String,
    }

    impl personal_info {
        pub fn new(new_edu: &str) -> Self {
            Self {
                education: String::from(new_edu),
                age: 20,
            }
        }
    }   
}

pub fn some_person() {
    let mut person1: personal_info = person::personal_info::new("Bachelors");
}
