# Rust Ownership and Borrowing Exercises

I've created a set of exercises for you to practice Rust's ownership and borrowing concepts. Each exercise focuses on a different aspect of these concepts. Here's a brief explanation of what each exercise is meant to teach:

## Exercises

### Basic Ownership

This exercise demonstrates how ownership is transferred when passing a variable to a function.

# E1

Now, let's go through what we've done in Exercise 1:

1. We created a "String" variable called `name` and initialized it with `Alice`.
2. We defined a function `print_name` that takes ownership of a `String` and prints it.
3. We called `print_name(name)`, which moves ownership of `name` into the function.
4. The last line is commented out because it would cause a compilation error. Once `name` is moved into `print_name`, it can no longer be used in `main`.

To see the ownership principle in action:

1. Copy this code and run it. It should compile and run without issues.
2. Now, uncomment the last line `(println!("Name is: {}", name);)` and try to compile it again.
3. You should see a compilation error saying that `name` has been moved and can't be used here.

This demonstrates how Rust's ownership system prevents use of variables after they've been moved.

### Borrowing

Here, you'll practice borrowing a value without taking ownership.

# E2

Now, let's go through what we've done in Exercise 2:

1. We created a vector of integers called `numbers`.
2. We defined a function `print_length` that takes a reference to a `Vec<i32>` (denoted by `&Vec<i32>`). This means it borrows the vector without taking ownership.
3. We called `print_length(&numbers)`, passing a reference to our vector.
4. After the function call, we print the vector itself.

Key points to note:

- The `&` in the function parameter `v: &Vec<i32>` indicates that we're borrowing the vector, not taking ownership.
- When calling the function, we use `&numbers` to create a reference to our vector.
- Because we only borrowed the vector, we can still use it in `main` after the function call.

This demonstrates how borrowing allows us to use a value in a function without moving ownership, which lets us continue using the value in the original scope.
To see this in action:

1. Copy this code and run it. It should compile and run without issues.
2. You'll see that the program prints both the length of the vector and the vector itself.
3. Try removing the `&` from both the function definition and the function call, and observe the compilation errors you get.

### Mutable Borrowing

This exercise shows how to mutably borrow a value to modify it.

# E3

Let's break down what we've done in Exercise 3:

1. We created a mutable vector of integers called `numbers` using `let mut numbers = vec![1, 2, 3, 4, 5];`.
2. We defined a function `add_element` that takes a mutable reference to a `Vec<i32>` (denoted by `&mut Vec<i32>`). This allows the function to modify the borrowed vector.
3. Inside `add_element`, we use `v.push(6)` to add a new element to the vector.
4. We call `add_element(&mut numbers)`, passing a mutable reference to our vector.
5. After the function call, we print the vector to see the changes.

Key points to note:

- The `mut` keyword is used both when declaring the vector (`let mut numbers`) and when passing it to the function (`&mut numbers`).
- The function parameter `v: &mut Vec<i32>` indicates that we're borrowing the vector mutably, allowing us to modify it.
- After calling the function, the changes made to the vector are visible in the main function.

This demonstrates how mutable borrowing allows us to modify a value in a function without taking full ownership, and have those modifications persist outside the function.

To see this in action:

1. Copy this code and run it. It should compile and run without issues.
2. You'll see that the program prints the vector both inside the function (after adding an element) and in the main function, showing that the modification persists.
3. Try removing the `mut` keyword from either the vector declaration or the function call, and observe the compilation errors you get.

### Multiple Borrows

This demonstrates how you can have multiple immutable borrows or one mutable borrow at a time.

# E4

Let's break down what we've done in Exercise 4:

We created a mutable String called greeting with the initial value "Hello".
We defined two functions:

print_length takes an immutable reference (&String) and prints the string's length.
add_world takes a mutable reference (&mut String) and appends " World!" to the string.

We first call print_length(&greeting), passing an immutable reference.
Then we call add_world(&mut greeting), passing a mutable reference.
Finally, we print the modified greeting in the main function.

Key points to note:

The print_length function only needs to read the string, so it takes an immutable reference.
The add_world function needs to modify the string, so it takes a mutable reference.
Rust allows us to have either multiple immutable borrows or one mutable borrow at a time, but not both simultaneously.
The mutable borrow in add_world is completed before we use greeting again in the final println!, so there's no conflict.

This demonstrates how Rust's borrow checker ensures memory safety by preventing data races at compile-time.
To see this in action and experiment further:

Run the code as is. It should compile and run without issues.
Try to call print_length(&greeting) immediately after add_world(&mut greeting). It will work because the mutable borrow has ended.
Try to create a mutable borrow that overlaps with an immutable borrow. For example, add these lines before calling add_world:
rustCopylet ref1 = &greeting;
let ref2 = &mut greeting; // This will cause a compile-time error
You'll see that Rust prevents this potential data race at compile-time.

### Ownership and Functions

This exercise shows how functions can take ownership and return values.

# E5

Let's break down what we've done in Exercise 5:

We defined a function string_length that takes ownership of a String and returns its length as a usize.
Inside string_length, we calculate the length, and then the owned String is dropped as it goes out of scope.
In main, we create a String called my_string.
We call string_length(my_string), which moves ownership of my_string into the function.
We store the returned length in the length variable.
We have a commented-out line that tries to use my_string after it's been moved.
Finally, we print the length.

Key points to note:

The string_length function takes ownership of the String (not a reference).
Once my_string is passed to string_length, it can no longer be used in main.
The function returns a new value (usize) which we can use.
This pattern is common when you want to consume a value and produce something else.

To see this in action and experiment further:

Run the code as is. It should compile and run without issues.
Uncomment the line // println!("My string is: {}", my_string); and try to compile the code. You'll see an error because my_string has been moved and is no longer available.
If you wanted to keep the original string, you could modify string_length to take a reference instead:
rustCopyfn string_length(s: &String) -> usize {
s.len()
}
And call it with let length = string_length(&my_string);. This would allow you to use my_string after the function call.

This exercise demonstrates how Rust's ownership system ensures memory safety by preventing use of moved values, while still allowing flexible patterns of data handling.

## Instructions

To practice, try implementing these exercises one by one. After each implementation, compile your code and see if it works as expected. If you encounter any errors, try to understand why they occur and how to fix them.

# Congratulations

Congratulations on completing all five exercises! You've covered a wide range of Rust's ownership and borrowing concepts.
