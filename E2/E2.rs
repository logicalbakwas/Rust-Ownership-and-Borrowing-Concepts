// Exercise 1: Basic Ownership
// ... (previous implementation remains the same)

// Exercise 2: Borrowing
fn main() {
    // Create a vector of integers
    let numbers = vec![1, 2, 3, 4, 5];

    // Create a function that borrows the vector and prints its length
    fn print_length(v: &Vec<i32>) {
        println!("The vector's length is: {}", v.len());
    }

    // Call the function
    print_length(&numbers);

    // Print the vector after the function call
    println!("The vector is: {:?}", numbers);
}

// Exercise 3: Mutable Borrowing
fn main() {
    // TODO: Create a mutable vector of integers
    // TODO: Create a function that mutably borrows the vector and adds an element to it
    // TODO: Call the function
    // TODO: Print the vector after the function call
}

// Exercise 4: Multiple Borrows
fn main() {
    // TODO: Create a String
    // TODO: Create two functions:
    //       1. One that takes an immutable reference to the String and prints its length
    //       2. Another that takes a mutable reference to the String and adds " World!" to it
    // TODO: Call both functions
    // TODO: Print the final String
}

// Exercise 5: Ownership and Functions
fn main() {
    // TODO: Create a function that takes ownership of a String and returns the String's length
    // TODO: Call the function and store the result
    // TODO: Try to use the original String after the function call and observe the error
    // TODO: Print the length
}
