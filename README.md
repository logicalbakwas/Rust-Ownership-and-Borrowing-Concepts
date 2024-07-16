# Rust Ownership and Borrowing Exercises

I've created a set of exercises for you to practice Rust's ownership and borrowing concepts. Each exercise focuses on a different aspect of these concepts. Here's a brief explanation of what each exercise is meant to teach:

## Exercises

### Basic Ownership

This exercise demonstrates how ownership is transferred when passing a variable to a function.

# E1

Now, let's go through what we've done in Exercise 1:

We created a String variable called name and initialized it with "Alice".
We defined a function print_name that takes ownership of a String and prints it.
We called print_name(name), which moves ownership of name into the function.
The last line is commented out because it would cause a compilation error. Once name is moved into print_name, it can no longer be used in main.

To see the ownership principle in action:

Copy this code and run it. It should compile and run without issues.
Now, uncomment the last line (println!("Name is: {}", name);) and try to compile it again.
You should see a compilation error saying that name has been moved and can't be used here.

This demonstrates how Rust's ownership system prevents use of variables after they've been moved.

### Borrowing

Here, you'll practice borrowing a value without taking ownership.

# E2

Now, let's go through what we've done in Exercise 2:

We created a vector of integers called numbers.
We defined a function print_length that takes a reference to a Vec<i32> (denoted by &Vec<i32>). This means it borrows the vector without taking ownership.
We called print_length(&numbers), passing a reference to our vector.
After the function call, we print the vector itself.

Key points to note:

The & in the function parameter v: &Vec<i32> indicates that we're borrowing the vector, not taking ownership.
When calling the function, we use &numbers to create a reference to our vector.
Because we only borrowed the vector, we can still use it in main after the function call.

This demonstrates how borrowing allows us to use a value in a function without moving ownership, which lets us continue using the value in the original scope.
To see this in action:

Copy this code and run it. It should compile and run without issues.
You'll see that the program prints both the length of the vector and the vector itself.
Try removing the & from both the function definition and the function call, and observe the compilation errors you get.

### Mutable Borrowing

This exercise shows how to mutably borrow a value to modify it.

### Multiple Borrows

This demonstrates how you can have multiple immutable borrows or one mutable borrow at a time.

### Ownership and Functions

This exercise shows how functions can take ownership and return values.

## Instructions

To practice, try implementing these exercises one by one. After each implementation, compile your code and see if it works as expected. If you encounter any errors, try to understand why they occur and how to fix them.
