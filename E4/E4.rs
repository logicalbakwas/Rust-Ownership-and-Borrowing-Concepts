// Exercise 1, 2, and 3: ... (previous implementations remain the same)

// Exercise 4: Multiple Borrows
fn main() {
    // Create a String
    let mut greeting = String::from("Hello");

    // Create two functions:
    // 1. One that takes an immutable reference to the String and prints its length
    fn print_length(s: &String) {
        println!("Length of the string: {}", s.len());
    }

    // 2. Another that takes a mutable reference to the String and adds " World!" to it
    fn add_world(s: &mut String) {
        s.push_str(" World!");
        println!("Modified string: {}", s);
    }

    // Call both functions
    print_length(&greeting);
    add_world(&mut greeting);

    // Print the final String
    println!("Final greeting: {}", greeting);
}

// Exercise 5: Ownership and Functions
fn main() {
    // TODO: Create a function that takes ownership of a String and returns the String's length
    // TODO: Call the function and store the result
    // TODO: Try to use the original String after the function call and observe the error
    // TODO: Print the length
}
