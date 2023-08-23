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
