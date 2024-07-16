// Exercise 1 and 2: ... (previous implementations remain the same)

// Exercise 3: Mutable Borrowing
fn main() {
    // Create a mutable vector of integers
    let mut numbers = vec![1, 2, 3, 4, 5];

    // Create a function that mutably borrows the vector and adds an element to it
    fn add_element(v: &mut Vec<i32>) {
        v.push(6);
        println!("Added an element. Vector in function: {:?}", v);
    }

    // Call the function
    add_element(&mut numbers);

    // Print the vector after the function call
    println!("Final vector: {:?}", numbers);
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