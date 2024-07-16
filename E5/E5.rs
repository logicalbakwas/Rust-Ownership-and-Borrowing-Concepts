// Exercise 1, 2, 3, and 4: ... (previous implementations remain the same)

// Exercise 5: Ownership and Functions
fn main() {
    // Create a function that takes ownership of a String and returns the String's length
    fn string_length(s: String) -> usize {
        let length = s.len();
        // s is dropped here, as it goes out of scope
        length
    }

    // Call the function and store the result
    let my_string = String::from("Hello, Rust!");
    let length = string_length(my_string);

    // Try to use the original String after the function call and observe the error
    // Uncomment the next line to see the compilation error:
    // println!("My string is: {}", my_string);

    // Print the length
    println!("The length of the string was: {}", length);
}
