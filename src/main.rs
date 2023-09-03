use std::hash;
use std::{f32::consts::PI, fmt::Arguments};
use rust_course::sec_8;

fn main() {
    // this is a comment!

    /*
    This is a 
    multiline comment.
     */
    println!("Hello, world!");
    print!("No new line");
    println!("{}", 1);

    print!("This will print 
                on multiple
                    line
                        just like this! \n");


    println!("My name is {firstname} {lastname} and I am {age} years old.", age="25", firstname="Suvineet", lastname="Singh");


    //  Variables: varibales are immutable by default
    let x:f32 = 15.0;
    println!("x = {}", x);

    let mut y:f32 = 12.9; // This is a mutable variable.
    println!("{}", y);
    y = 10.9;

    // Scalar types
    /*
    Interger
        - Signed: i8, i16, i32, i64
        - Unsigned: u8, u16, u32, u64
    

    
     */

    println!("Max number in i8 = {}", std::i8::MAX);
    println!("Min number in u8 = {}", std::u8::MIN);

    /* 
    Boolean
     */
    let status = false;
    println!("The values of variables are {:?}", (x, y, status));

    let not_equals: bool = 18 != 18;
    println!("The condition is {}", not_equals);

    let _x2:i32;

    
    let (_first_num, _second_num) = (250, 480.99);
    let _large_number = 1_000_000;

    let n1: i32 = 14;
    let n2: f64 = 15.6;

    let n3: i32 = n1 + n2 as i32; // Here n2 is not mutated, just used as i32 for this statement.
    println!("n1={n1}, n2={n2}, n3={n3}");



    // Shadowing
    let s = 5;
    let s = 5 * 5;
    println!("{}", s);


    let r = 65;
    {
        let r = 25;
        println!("Inside the code segment = {r}");
    }
    println!("Outside code segment = {r}");


    // Constants
    // Not same as immutable variables. The data type MUST be defined and cannot be inferred.
    const PI: f32 = 3.14;
    println!("PI = {PI}");


    // Strings
    /*
        - Fixed length strings (&str) - string slices, cannot be mutated, fixed size, they are references.
        - Growable strings (String)
     */

    let _fixed_string: &str = "Fixed length string";
    let mut growable_string: String = String::from("This string can mutate");
    println!("Growable String = \"{growable_string}\"");
    
    growable_string.push('!');
    println!("Growable String = \"{growable_string}\"");
    
    growable_string.pop();
    println!("Growable String = \"{growable_string}\"");
    
    growable_string.push_str("Added string");
    println!("Growable String = \"{growable_string}\"");

    println!(
        "Basic functions on Strings,
        is_empty() - check if empty: {},
        len() - Length of string: {},
        capacity() - Bytes used in memory: {},
        contains() - If string contains this : {}",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("mutate")
    );

    let number: i32 = 6;
    let number_string = number.to_string();
    println!("{}", number_string);

    let empty_string = String::new();
    println!("Length is {}", empty_string.len());

    let s_1: String = "Suvineet".to_string();
    let s_2: String = "Singh".to_string();
    let s_3: String = format!("My first name is {s_1} and last name is {s_2}");

    println!("s3 = {}", s_3);

    // Tuples
    let my_info: (&str, i32) = ("Salary", 50_000);
    println!("{} is equal to {}", my_info.0, my_info.1);

    println!("Another way of printing tupe {:?}", my_info);

    let nested_tuple: (i32, f64, (i32, i32), &str) = (4, 5.0, (3, 2), "Hello");
    let _element: i32 = nested_tuple.2.0;

    let _empty_tuple: () = ();

    // Arrays
    let mut number_array: [i32;5] = [4, 5, 6, 8, 9];
    println!("{}", number_array[0]);
    println!("{:?}", number_array);

    // Update the 5th element
    number_array[4] = 5;

    let _array_with_same_elements: [i32;10] = [0; 10]; // 10 elements value 0

    let mut string_array_1: [&str; 3] = ["apple", "tomato", "grapes"];

    let _string_array_2: [&str; 6] = ["Unknown"; 6];
    string_array_1[0] = "suvineet singh";

    let _char_array: [char; 4] = ['s', 'u', 'v', 'i'];

    let number_array_1: [i32;5] = [4, 5, 6, 7, 8];

    let subset_array: &[i32] = &number_array_1[0..3]; // arrays slices which are references.
    println!("Subset Array of {:?} is {:?}", number_array_1, subset_array);

    println!("Number of elements in array = {}", number_array_1.len());
    println!("The array is occupying {} bytes", std::mem::size_of_val(&number_array_1)); // i32 is 4 bytes each so should 20 bytes.

    let check_index: Option<&i32> = number_array_1.get(2);

    println!("{:?}", check_index);
    test(12, 54.0, "idk");

    let dist = cartesian_distances_tuple((5, 5), (0, 0));
    println!("Distance in X = {}, Y = {}", dist.0, dist.1);

    let dist: [f64;2] = cartesian_distances_array([45.0, 1.0], [-78.0, 23.0]);
    println!("Distance in X = {}, Y = {}", dist[0], dist[1]);

    let point1 = (0, 0);
    let point2 = (2, 0);
    println!("Euclidean Distance between {:?} and {:?} = {:?}", point1, point2, eucledian_dist(point1, point2));


    // Vectors - Arrays that can be resized. Has to be same type
    let mut number_vec: Vec<i32> = vec![4, 5, 6, 7,8, 9, 10, 11, 12, 13];
    println!("{:?}", number_vec);
    number_vec[4] = 25;
    println!("{:?}", number_vec);

    let _array_with_same_elements: Vec<i32> = vec![0;10];

    let mut string_array_1: Vec<&str> = vec!["apple", "mango", "grapes"];
    let _string_array_2: Vec<&str> = vec!["Unknown";6];
    string_array_1[0] = "Something new";

    let _char_vec: Vec<char> = vec!['s', 'u', 'v', 'i'];

    let subset_vec: &&[i32] = &&number_vec[0..3];
    println!("The subset of values of the array are {:?}", subset_vec);

    number_vec.push(50);
    println!("{:?}", number_vec);

    number_vec.remove(3);
    println!("{:?}", number_vec);

    println!("The value of 10 exists? {}", number_vec.contains(&10));

    // User inputs
    let mut user_input: String = String::new();
    std::io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input.");
    
    let user_input: f64 = user_input.trim().parse().expect("Invalid Input");
    println!("You typed '{}'", user_input);

    ownership();
    references();
    if_statements();
    match_statement();
    loops();
    square_difference();
    sum_of_3_5_divisible();
    cars_produced_per_minute();
    let phrase: &str = "tenet";
    println!("'{}' is a palindrome = {}", phrase, is_palindrome(phrase));

    structures();
    tuple_structures();
    traits();
    enums();
    generics();
    option_enum();
    result_enum();
    hash_maps();
    library_mgmt();
    student_mgr_assignment();
    closures();
    function_types();
    iterators();
    sets_assigment();

    sec_8::smart_pointers();
    sec_8::linked_list();
    sec_8::deref_coersion();
    sec_8::rc_pointers();
    sec_8::doubly_linked_list();
    sec_8::weak_and_strong_refs()

}


fn test(a: i32, b: f64, c:&str) {
    println!("{a}, {b}, {c}");
}

fn cartesian_distances_tuple(coord1: (i32, i32), coord2: (i32, i32)) -> (i32, i32){
    let x_dist: i32 = (coord1.0 - coord2.0).abs();
    let y_dist: i32 = (coord1.1 - coord2.1).abs();
    return (x_dist, y_dist);
}

fn cartesian_distances_array(coord1: [f64;2], coord2: [f64;2]) -> [f64;2] {
    let x_dist: f64 = (coord1[0] - coord2[0]).abs();
    let y_dist: f64 = (coord1[1] - coord2[1]).abs();
    return [x_dist, y_dist];
}


fn eucledian_dist(coord1: (i32, i32), coord2: (i32, i32)) -> f32 {
    (((coord1.0 - coord2.0).pow(2) + (coord1.1 - coord2.1).pow(2)) as f32).sqrt()
    // A function can have 1 statement without a ; which is assumed to be the returned value.
}



/*
RUST OWNERSHIP
    - Each value in Rust has a variable thats called its owner
    - There can only be one owner at a time
    - When the owner goes out of scope, the value will be dropped.
*/

fn ownership() {
    let x: f64 = 32.6;
    let y: f64 = x; // A copy is made i.e. new location in memory.
    // These are primitive (i.e. cannot be changed) fixed in size. (stack)

    println!("x: {}, y: {}", x, y);

    let s1: String = String::from("abc"); // assigns some memory containing value "abc" owned by s1
    // let s2: String = s1; // Not a copy, moves the value in memory to the variable s2. s1 is not valid anymore.
    // Non primitive type: computation is required to see how to assign memory when they change etc. (heap)

    // Therefore, ownership needs to be thought of with non-primitives.

    let s2: &String = &s1; // Reference to value of s1 and doesnt change ownership.
    // Referencing doesnt change ownership, this is borrowing value.
    println!("s1: {}, s2: {}", s1, s2);


    let vec_1: Vec<i32> = vec![1, 2, 3, 4, 5];
    let vec_2: &Vec<i32> = &vec_1;
    println!("{:?}, {:?}", vec_1, vec_2);

    let vec_3: Vec<i32> = vec_1.clone(); // Here we make a new copy of vec_1 in new memory.
    println!("{:?}", vec_3);
    

    let stack_num: i32 = 32;
    let mut heap_vec: Vec<i32> = vec![4, 5, 6];
    stack_function(stack_num);
    println!("The value inside the main of stack_num: {}", stack_num);

    /* Here we pass in a mutable reference to heap_vec. i.e. The ownership remains with heap_vec
    the function will get a reference to heap_vec which is mutable.
    
    If we pass in heap_vec without &, then the ownership will pass to the the variable defined
    inside the function. Hence, when the function finishes, the ownership will be out of scope,
    and the value will be dropped -> head_vec will no longer have a value.

    If we pass &heap_vec without the &mut then we cannot make changes to the value this reference
    inside the called function because this reference is not mutable.
    */
    heap_functon(&mut heap_vec);
    println!("Value inside the main of heap_vec: {:?}", heap_vec);

}


fn stack_function(mut stack_num:i32) {
    stack_num = 56;
    println!("Var: {}", stack_num);
}

fn heap_functon(var: &mut Vec<i32>) {
    /* 
    Here we receive a mutable reference which means ownership isnt passed to var.
    var is just a reference which is also mutable which can mutate the original
    value stored in the variable who's reference is passed in.
    */
    var.push(100);
    println!("Heap Var: {:?}", var);
}



/*
REFERENCES RULES:
    - One mutable reference in a scope
    - Many immutable references
    - Mutable and immutable cannot coexist within a scope.
    - Scope of a reference (scope is not just code block. Definition to Last usage).
    - Data should not change when immutable references are in scope
*/

fn references() {
    let mut heap_num: Vec<i32> = vec![4, 5, 6];
    let _ref1: &mut Vec<i32> = &mut heap_num; // Cannot have another mutable reference to this in this scope.

    let mut heap_num: Vec<i32> = vec![4, 6, 7];
    let iref1: &Vec<i32> = &heap_num;
    let iref2: &Vec<i32> = &heap_num; // Can have as many immutable references.
    println!("{:?}, {:?}", iref1, iref2);

    let mut heap_num: Vec<i32> = vec![3, 4, 5];
    let iref1: &Vec<i32> = &heap_num;
    let iref2: &Vec<i32> = &heap_num; // scope of ref2 starts here!
    println!("{:?}, {:?}", iref1, iref2); // scope of ref2 ends here (last usage)

    // Since scope of ref2 is done. We are out of scope so we can now define a mutable reference.
    let mref1: &mut Vec<i32> = &mut heap_num;
    println!("{:?}", mref1);

    let mut heap_num: Vec<i32> = vec![3, 4, 5];
    let iref1: &Vec<i32> = &heap_num;
    let iref2: &Vec<i32> = &heap_num;

    // heap_num.push(68); // Cannot change this data as iref1 in scope.
    println!("{:?}, {:?}", iref1, iref2);

}


// CONTROL STRUCTURES
fn if_statements() {
    let some_number: i32 = 40;
    let another_number: i32 = 45;
    if some_number <= 50 && another_number < 55 {
        println!("Yes!");
    }

    let flag_1: bool = false;
    let flag_2: bool = true;

    if flag_1 || flag_2 {
        println!("Hi!")

    } else if another_number > 55 {
        println!("Hello!");
    } else {
        println!("Bye!");
    }


    // if let
    let marks: i32 = 95;
    let grade: char = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else {
        'F'
    };
    println!("Grade = {}", grade);


}

fn match_statement() {
    // This is like switch case statement.
    let some_number: i32 = 100;
    match some_number {
        1 => println!("The number is 1"),
        2 | 3 => println!("The number is 2 or 3"),
        4..=100 => println!("The number is between 4 and 100"),
        _ => println!("The number is greater than 100"),
    }

    let marks: i32 = 95;
    let mut _grade: char = match marks {
        90..=100 => {
            println!("WOOHOO!");
            'A'
        },
        80..=89 => 'B',
        _ => 'F',
    };
}

fn loops() {
    /*
    loop {
        println!("Infinite loop!")
    }
    */

    // WHILE LOOOPS

    let my_number: u8 = 5;
    let mut guess: bool = false;
    println!("Guess my number between 1 and 20");

    while !guess {
        let mut input: String = String::new();
        std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
        let input: u8 = input.trim().parse().expect("invalid input");
        guess = input == my_number;
    };

    println!("Enter a number and I will tell you next number that is divisible by 2 and 5");
    let mut input_number: String = String::new();
    std::io::stdin()
    .read_line(&mut input_number)
    .expect("Failed to read input");

    let mut number: u8 = input_number.trim().parse().expect("Invalid input");
    number += 1;
    while !(number % 2 == 0 && number % 5 == 0) {
        number += 1;
    }
    println!("next number div by 2 and 5 is: {}", number);


    // FOR LOOPS
    let some_vec: Vec<i32> = vec![45, 34, 85, 89, 192, 981];
    for i in 0..some_vec.len() {
        println!("{}", some_vec[i]);
    }

    for i in some_vec {
        println!("{}", i);
    } // Here the ownesrhip will go to i so after loop some_vec will be ded.

    let mut some_vec: Vec<i32> = vec![45, 34, 85, 89, 192, 981];
    for i in some_vec.iter() { // can also do &some_vec
        println!("{}", i);
    }
    println!("I can use some_vec here because iter doesnt change ownership: {:?}", some_vec);

    for i in some_vec.iter_mut() { // can also do &mut some_vec
        *i += 5;
    }

    println!("----------");
    let mut var: i32 = 13;
    loop {
        var -= 1;
        if var % 13 == 0 {
            continue;
        }
        if var == 1{
            break;
        }
        println!("{}", var);
    }

}

fn square_difference() {
    let mut input: String = String::new();
    println!("Enter N for first N integers sum of squares difference: ");
    std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");

    let input_number: u32 = input.trim().parse().expect("Invalid input number.");

    let sum_squared: u32 = {
        let mut sum: u32 = 0;
        for i in 1..=input_number {
            sum += i;
        };
        sum.pow(2)
    };

    let sum_of_squares: u32 = {
        let mut sum: u32 = 0;
        for i in 1..=input_number {
            sum += i.pow(2);
        }
        sum
    };
    println!("Difference between the Square of sum and sum of squares upto {} = {}", input_number, sum_squared - sum_of_squares);
}

fn sum_of_3_5_divisible() {
    let mut input: String = String::new();
    println!("Enter N for sum of numbers divisible by 3 or 5: ");
    std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");

    let input_number: u32 = input.trim().parse().expect("Invalid input number.");
    
    let mut sum: u32 = 0;

    for i in 1..input_number {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    println!("Sum of multiples of 3 and 5 upto {} = {}", input_number, sum);
}

fn total_production(hours: f32, speed: u8) -> i32 {
    let success_ratio: f32 = match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    };

    let cars_produced: i32 = (221.0 * hours * speed as f32 * success_ratio) as i32;
    println!("Cars produced in {} hrs at speed {} = {}", hours, speed, cars_produced);
    cars_produced
}

fn cars_produced_per_minute() {
    let mut input: String = String::new();

    println!("Enter the number of hours and speed as (h, s): ");
    std::io::stdin()
    .read_line(&mut input)
    .expect("Failed to read input");
    
    let mut input_vals: Vec<&str> = vec![];
    for item in input.split_whitespace() {
        input_vals.push(item);
    };
    let hours: f32 = input_vals[0].parse().expect("Invalid input hours");
    let speed: u8 = input_vals[1].parse().expect("Invalid input speed");

    let total_cars_produced: i32 = total_production(hours, speed);
    println!("Cars produced per minute = {}", total_cars_produced as f32 / hours / 60.0);
}

fn is_palindrome(phrase: &str) -> bool {
    let mut phrase_as_vec: Vec<char> = vec![];
    for character in phrase.chars() {
        phrase_as_vec.push(character);
    };

    for i in 0..phrase_as_vec.len() / 2 {
        if phrase_as_vec[i] != phrase_as_vec[phrase_as_vec.len() - 1 - i] {
            return false;
        }
    }
    true
}

// STRUCTS BASICS
struct Person {
    name: String,
    age: u8,
    gender: char,
    salary: i32,
}

// Class methods etc can be implemented here
impl Person {
    fn compute_taxes(&self) -> f32 {
        0.2 * self.salary as f32
    }

    // This is like adding a __init__ for this stuct
    fn new() -> Self {
        Person { name: String::from("default"), age: 0, gender: 'M', salary: 10_000 }
    }

    fn static_method(param: f32) -> f32 {
        0.4 * param
    }
}

fn structures() {
    let person_1: Person = Person{
        name: String::from("Suvineet Singh"),
        age: 25,
        salary: 50_000,
        gender: 'M',
    };

    println!("Taxes for {} at salary {} = {}", person_1.name, person_1.salary, person_1.compute_taxes());
    println!("{}", Person::static_method(3.0));

    let mut default_person: Person = Person::new();
    println!("Constructor with some defaults: {} {}", default_person.name, default_person.age);
    default_person.age = 20;
    println!("Constructor with some defaults: {} {}", default_person.name, default_person.age);

    let person_3: Person = Person { 
        name: String::from("Person3"), 
        age: 10,
        ..person_1 // here we are initialising person_3 with the rest of attributes from person_1 (Base construct)
    };
    println!("Person3 = {} {}", person_3.name, person_3.gender);
}

// TUPLE STRUCTURE
// simple tuples do not have an associated name, but tuple structs do.
struct Numbers(i32, i32);

impl Numbers {
    fn greater(&self) -> i32 {
        if self.0 >= self.1 { self.0 } else { self.1 } 
    }

    fn lesser(&self) -> i32 {
        if self.0 <= self.1 { self.0 } else { self.1 }
    }
}

fn tuple_structures() {
    let some_nums: Numbers = Numbers(12, 24);
    println!("Some numbers = {}, {}", some_nums.0, some_nums.1);
    println!("Greater = {}, Lesser = {}", some_nums.greater(), some_nums.lesser());
}


// Traits
struct Student {
    name_std: String,
    age_std: u8,
    sex: char,
}

trait GeneralInfo {
    fn info(&self) -> (&str, u8, char); // just add function defs in trait not implementation.

    fn gender_info(&self) -> char;
}

impl GeneralInfo for Person {
    fn info(&self) -> (&str, u8, char) {
        (&self.name, self.age, self.gender)
    }

    fn gender_info(&self) -> char {
        self.gender
    }
}

impl GeneralInfo for Student { // all methods of trait must be implemented.
    fn info(&self) -> (&str, u8, char) {
        (&self.name_std, self.age_std, self.sex)
    }

    fn gender_info(&self) -> char {
        self.sex
    }
}

fn traits() {
    let person_1: Person = Person{
        name: String::from("Suvi"),
        age: 25,
        gender: 'M',
        salary: 50_000,
    };

    let student_1: Student = Student { name_std: String::from("Student"), age_std: 30, sex: 'F' };
    println!("Person info: {:?}", person_1.info());
    println!("Student info: {:?}", student_1.info());

    let circle: Circle = Circle{
        radius: 3.0
    };
    let rectangle: Rectangle = Rectangle{
        length: 2.0,
        width: 3.0,
    };

    println!("Area of circle with radius {} = {}. Aread of Rectangle with dimensions {:?} = {}", circle.radius, circle.area(), (rectangle.length, rectangle.width), rectangle.area());
    println!("Circumference of circle with radius {} = {}. Perimeter of Rectangle with dimensions {:?} = {}", circle.radius, circle.perimeter(), (rectangle.length, rectangle.width), rectangle.perimeter());
    circle.default_function();
    println!("circle = {}, rectangle = {}", circle.default_function(), rectangle.default_function());
}

struct Circle {
    radius: f32,
}

struct Rectangle {
    length: f32,
    width: f32,
}

struct Square {
    width: f32,
}

trait ShapeInfo {
    fn area(&self) -> f32;
    fn perimeter(&self) -> f32;
    fn default_function(&self) -> &str {
        "I am implemented in the trait definition but I am not implemented for this type."
    }
}

impl ShapeInfo for Circle {
    fn area(&self) -> f32 {
        PI *self.radius.powf(2.0)
    }

    fn perimeter(&self) -> f32 {
        PI * 2.0 * self.radius
    }

    fn default_function(&self) -> &str {
        "I am a circle!"
    }
}

impl ShapeInfo for Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn perimeter(&self) -> f32 {
        2.0 * (self.length + self.width)
    }
}

impl ShapeInfo for Square {
    fn area(&self) -> f32 {
        self.width.powf(2.0)
    }

    fn perimeter(&self) -> f32 {
        4.0 * self.width
    }

    fn default_function(&self) -> &str {
        "I am a square!"
    }
}

// Enums

enum Colour {
    Green = 12,
    Red, // This will be 13
    Yellow, // This will be 14
    Blue = 10, 
    Black, // This will try to be 9
    
}

enum Vehicle {
    Car = 15,
    Train,
    Bus = 20
}

impl Vehicle {
    fn travel_allowance(&self, miles: i32) -> f32 {
        match self {
            Vehicle::Car => miles as f32 * 14.0,
            Vehicle::Train => miles as f32 * 18.0,
            Vehicle::Bus => miles as f32 * 12.0,
        }
    }
}


fn enums() {
    println!("Green = {:?}", Colour::Green as i32);
    println!("Red = {:?}", Colour::Red as i32);
    println!("Yellow = {:?}", Colour::Yellow as i32);
    println!("Blue = {:?}", Colour::Blue as i32);

    let car: Vehicle = Vehicle::Car;
    let bus: Vehicle = Vehicle::Bus;
    let train: Vehicle = Vehicle::Train;
    println!("Travel allowances. Car: {}, Train: {}, Bus: {}", car.travel_allowance(12), train.travel_allowance(12), bus.travel_allowance(12));

    // We can also store values in enum values.
    let child: AgeGroup = AgeGroup::Child(12);
    let adult: AgeGroup = AgeGroup::Adult(21);
    let senior: AgeGroup = AgeGroup::Senior(65);
    println!("Ticket prices. child = {}, adult = {}, senior = {}", child.ticket_price(), adult.ticket_price(), senior.ticket_price());

    let some_val: Vec<Value> = vec![Value::Integer(12), Value::Float(2.0)];
    println!("The value of the integer is {:?} and float is {:?}", some_val[0], some_val[1]);

    for i in some_val.iter() {
        match i {
            Value::Integer(num) => println!("The value of the integer is {}", *num),
            Value::Float(num) => println!("The value of the float is {}", *num),
        }
    }
}

enum AgeGroup {
    // Here we are attaching some data of type u32 to each enum value (date being age)
    Child(u8),
    Adult(u8),
    Senior(u8),
}

impl AgeGroup {
    fn ticket_price(&self) -> f32 {
        match self {
            AgeGroup::Child(age) => *age as f32 * 1.0,
            AgeGroup::Adult(age) => *age as f32 * 1.5,
            AgeGroup::Senior(age) => *age as f32 * 1.1,
        }
    }
}

#[derive(Debug)] // Here we allow enum to use the Debug Trait which is used for printing stuff.
enum Value {
    Integer(i32),
    Float(f32),
}


// GENERICS

/* 
Here we take in a variable of generic type T.
We also restrict T to be of types that implement the 
traits of Multiplication and Copy resulting in an output of type T.
Primitive types are Copied and not Moved.
*/
fn square<T: std::ops::Mul<Output = T> + Copy> (x: T) -> T {
    x * x
}


/*
Here the restricitons in the definition may get too long. So another way to define
is by using the 'where' keyword.
*/
fn square_with_where_keyword<T> (x: T) -> T
where T: std::ops::Mul<Output = T> + Copy {
    x * x
}

/* 
Here we define Point with 2 generic types which each x,y can take
Allowing for point such as (2, 3.4) rather than both having to be
of the same type.
*/

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T,U> 
where T: std::fmt::Debug, U: std::fmt::Debug {
    fn print(&self) {
        println!("The value of the point coordinates are {:?}, {:?}", self.x, self.y);
    }
}

fn generics() {
    println!("square i32: {}", square(2 as i32));
    println!("square f32: {}", square(2.0 as f32));
    println!("square u8: {}", square_with_where_keyword(2 as u8));

    let point: Point<i32, f32> = Point{x: 2, y: 3.4};
    point.print();
}


/*
Option Enum defined in Rust as:

enum Option<T> {
    None,
    Some(T),
}
*/


fn option_enum() {
    let mut disease: Option<String> = None;
    disease = Some(String::from("Diabetes"));
    match disease {
        Some(disease_name) => println!("You have the disease of {}", disease_name),
        None => println!("You dont have a disease!"),
    }

    let s1: Option<&str> = Some("Some string");
    println!("The value of s1 is {:?} and the value of s1 itself after unwrapping is {:?}", s1, s1.unwrap());


    let f1: Option<f64> = Some(10.54);
    let mut f2: f64 = 16.5;
    f2 += f1.unwrap();
    println!("The value of the sum is {}", f2);

    let number: Option<i32> = Some(6);
    if square_with_option(number) != None {
        println!("The result of the square is {:?}", square_with_option(Some(6)).unwrap());
    } else {
        println!("We dont have a value");
    }
}


fn square_with_option(num: Option<i32>) -> Option<i32> {
    match num {
        Some(number) => Some(number * number),
        None => None,
    }
}


/*
Result Enum defined in Rust as:

enum Result<T, E> {
    Ok(T),
    Err(E),
}
*/

fn division(divident: f64, divisor: f64) -> Result<f64, String> {
    if divisor == 0.0 {
        Err(String::from("Error: Division by zero"))
    } else {
        Ok(divident / divisor)
    }
}

fn result_enum() {
    println!("4.0 / 3.0 = {:?}", division(4.0, 3.0));
    println!("3.0 / 0.0 = {:?}", division(3.0, 0.0));
    println!("0.0 / 1.0 = {:?}", division(0.0, 1.0));

    let some_vec: Vec<i32> = vec![5, 5, 2, 1, 5, 9];
    let result1: Result<&i32, &str> = match some_vec.get(5) {
        Some(a) => Ok(a),
        None => Err("The value doesn't exist"),
    };
    println!("The value of result is {:?}", result1);
}


// Hash Maps
use std::collections::HashMap;
fn hash_maps() {
    let mut person:HashMap<&str, i32> = HashMap::new();
    person.insert("Suvi", 25);
    person.insert("John", 45);

    println!("The age is {:?}", person.get("Suvi").unwrap());

    if person.contains_key("John") {
        println!("The key exists");
    } else {
        println!("The key doesnt exist");
    }

    match person.get("Alex") {
        Some(age) => println!("The key exists with value = {}", age),
        None => println!("Key doesnt exist")
    }

    for (name, age) in &person {
        println!("The person {} is {} years old", name, age);
    }

    for item in &person {
        println!("{:?}", item);
    }

    for key in person.keys() {
        println!("{:?}", key);
    }

    let mut hash_map: HashMap<&str, &str> = HashMap::new();
    hash_map.insert("Apple", "Red");
    hash_map.insert("Apple", "Green");

    hash_map.entry("Apple").or_insert("Black"); // only add new value if key doesnt exist.
    println!("hash_map = {:?}", hash_map);


    let some_vec: Vec<i32> = vec![5, 5, 7, 1, 5, 1, 1, 9, 1, 9, 5];
    let mut freq_vec:HashMap<i32, u32> = HashMap::new();

    for i in &some_vec { // same as some_vec.iter()
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1; // Same reference to the entry each time is incremented here.
    }
    println!("{:?}", freq_vec);

}


struct Item {
    id: i32,
    title: String,
    year: u32,
    item_type: ItemType,
}

#[derive(Debug)]
enum ItemType {
    Book,
    Magazine,
}

fn display_item_info(item: Item) {
    println!("ID = {}", item.id);
    println!("Title = {}", item.title);
    println!("Year = {}", item.year);
    println!("Type {:?}", item.item_type);
    println!("------------------");
}

fn library_mgmt() {
    display_item_info(
        Item{
            id: 0,
            title: String::from("book1"),
            year: 2020,
            item_type: ItemType::Book,
        }
    );
    display_item_info(
        Item{
            id: 1,
            title: String::from("magazine1"),
            year: 2020,
            item_type: ItemType::Magazine,
        }
    )
}



#[derive(Debug)]
struct AStudent {
    id: i32,
    name: String,
    grade: String,
}

struct AStudentManager {
    students: HashMap<i32, AStudent>
}

impl AStudentManager {
    fn new() -> Self {
        AStudentManager{students: HashMap::new()}
    }
    
    fn add_student(&mut self, student: AStudent) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            Result::Err(format!("Student with id {} already exists", student.id))
        } else {
            self.students.insert(student.id, student);
            Result::Ok(())
        }
    }

    fn get_student(&self, id: i32) -> Option<&AStudent> {
        self.students.get(&id)
    }
}

fn student_mgr_assignment() {
    let mut student_mgr: AStudentManager = AStudentManager::new();
    let student1: AStudent = AStudent{
        id: 0,
        name: String::from("John"),
        grade: String::from("Grade 10"),
    };
    let student2: AStudent = AStudent{
        id: 0,
        name: String::from("Alex"),
        grade: String::from("Grade 8"),
    };

    let mut students: Vec<AStudent> = Vec::new();
    students.push(student1);
    students.push(student2);

    for student in students {
        match student_mgr.add_student(student) {
            Ok(()) => println!("Student added!"),
            Err(err) => println!("Failed to add student. Error={}", err),
        }
    }

    match student_mgr.get_student(1) {
        Some(student) => println!("Get id : {:?}", student),
        None => println!("Student with id 1 not found."),
    }
}


// LIFETIMES
/*
Why we need lifetimes?
*/

fn lifetimes() {

    /*
    let i: &i32;
    {
        let j: i32 = 5;
        i = &j;
    }
    println!("The value of i = {}", i); // here we are referring to j which is now out of scope and is freed. This is a dangling reference.
    */

}


// CLOSURES
// syntax: |...| {...}


fn closures() {

    let x = 5;
    let closure1 = || println!("This is a print inside this closure without any input values");
    closure1();

    let closure2 = |num: i32| println!("I am printing the input inside this closure = {}", num);
    closure2(x);

    let closure3 = |general_info: String, name: &str, age: i32| { println!("{} {} {}", general_info, name, age) };
    let general_info: String = String::from("The details are ");
    let (person_name, person_age) = (String::from("Suvi"), 25);
    closure3(general_info, &person_name, person_age); // Here values of general_info and person_age has moved but person_name hasnt

    let closure4 = |num| num*num;
    let x = 5;
    closure4(x); // Even though closure input hasnt specified type, it is inferred from first call and is then set in stone.

    let division_status = |y: f32| { if y != 0.0 { true } else { false }};
    division_closures(5.0, 0.0, division_status);
    division_closures(5.0, 2.0, division_status);


    let mut vec_1: Vec<i32> = vec![1, 2, 3];
    let some_closure = || {
        println!("Vec 1: {:?}", vec_1); // here rust infers vec_1 as an immutable ref because we arent changing it.
    };
    println!("Vec 1: {:?}", vec_1);
    some_closure(); // This is where the scope of the immutatble ref to vec_1 ends.

    let mut mutable_closure = || { // here the closure must be mut because closure is mutating another value.
        vec_1.push(35);
    };
    // println!("{:?}", vec_1); This wont be allowed because closure has borrowed the mutable ref of vec_1
    // vec_1.push(1) Again not allowed for same above reason.
    mutable_closure();

    let move_value_closure = || {
        let vec_2: Vec<i32> = vec_1; // Here compiler infers that ownership of vec_1 is given to to vec_2 so it is done by value not referece.
    };

    move_value_closure();
    // println!("Vec 1: {:?}", vec_1); vec_1's value is dropped as it is owned by vec_2 which is now out of scope.
    // println!("Vec 2: {:?}", vec_2); vec_2 is out of scope here and is dropped.


}

fn division_closures<F: Fn(f32) -> bool>(x : f32, y: f32, f: F) {
    if f(y) {
        println!("The division result is {}", x / y);
    } else {
        println!("Division by zero not possible");
    }
}


// FUNCTION POINTER TYPES

fn function_types() {
    let f: fn(i32, i32) -> i32 = min;
    println!("the min of the two values is {:?}", f(2, 3));
    let f: fn(i32, i32) -> i32 = max;
    println!("the min of the two values is {:?}", f(2, 3));

    let (my_name, my_age) = (String::from("Suvi"), 25);
    prints_full_info(prints_name, my_name.as_str(), my_age);

    let answer: i32 = do_twice(add_one, 5);
    println!("Answer = {}", answer);
}

fn max(x: i32, y: i32) -> i32 {
    if x > y { x } else { y }
}

fn min(x: i32, y: i32) -> i32 {
    if x < y { x } else { y }
}

fn prints_name(name: &str) {
    println!("Name is {}", name);
}

fn prints_full_info(f: fn(&str), some_one: &str, age: i32) {
    f(some_one);
    println!("Age = {}", age);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}


// ITERATORS

fn iterators() {
    let some_vec: Vec<i32> = vec![1, 2];
    let mut iter: std::slice::Iter<'_, i32> = some_vec.iter();
    println!("The iterator = {:?}", iter);

    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    let a: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
    let check: bool = a.iter().any(|&x| x > 0);
    println!("Value of the any function is = {}", check);

    let check: bool = a.iter().all(|&x| x > 0);
    println!("Value of the all function is = {}", check);

    let check: Option<&i32> = a.iter().find(|&&x| x > 0);
    println!("The value of the find function is {:?}", check);

    let check: Option<usize> = a.iter().position(|&x| x > 4);
    println!("The value of the position function is {}", check.unwrap());

    let check: Option<usize> = a.iter().rposition(|&x| x > 4);
    println!("The value of the rposition function is {}", check.unwrap());

    let check: Option<&i32> = a.iter().max();
    println!("The value of the max function is {}", check.unwrap());

    let check: Option<&i32> = a.iter().min();
    println!("The value of the min function is {}", check.unwrap());

    let mut iter: std::iter::Rev<std::slice::Iter<'_, i32>> = a.iter().rev();
    println!("iter.next = {:?}", iter.next());

    let a: Vec<u32> = vec![1, 2, 3, 4, 5, 6];
    let filtered_values: Vec<&u32> = a.iter().filter(|&x| *x > 4).collect::<Vec<&u32>>();
    println!("{:?}", filtered_values);

    let b: Vec<u32> = a.clone();
    // into_iter uses the actual values so it moves the values in 'a' and hence 'a' is dropped.
    let filtered_values: Vec<u32> = a.into_iter().filter(|&x| x > 4).collect::<Vec<u32>>();
    println!("{:?}", filtered_values);

    let mut mapped_values = b.iter().map(|x| 2 * *x).collect::<Vec<u32>>();
    println!("{:?}", mapped_values);

    let mut mapped_values = b.iter().map(|x| 2 * *x).filter(|x| *x > 8).collect::<Vec<u32>>();
    println!("{:?}", mapped_values);

}


fn sets_assigment() {
    let vec_1: Vec<u32> = vec![5, 4, 3, 6, 9];
    let vec_2: Vec<u32> = vec![5, 8, 6, 4, 10, 15, 20, 21];
    let intersect = intersection(&vec_1, &vec_2);
    println!("Intersection = {:?}", intersect);
    let uni = union(&vec_1, &vec_2);
    println!("Union = {:?}", uni);
}

fn intersection(v1: &Vec<u32>, v2: &Vec<u32>) -> Vec<u32>{
    let intersect: Vec<u32> = v1.iter().filter(|&x| v2.contains(x)).cloned().collect();
    intersect
}

fn union(v1: &Vec<u32>, v2: &Vec<u32>) -> Vec<u32> {
    let mut union_set: Vec<u32> = v1.clone();
    let v2_without_v1: Vec<u32> = v2.iter().filter(|&x| !v1.contains(x)).cloned().collect();
    for item in v2_without_v1 {
        union_set.push(item);
    }
    union_set
}