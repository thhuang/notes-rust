fn main() {
    // Ownership Rules
    //   - Each value in Rust has a variable that’s called its owner.
    //   - There can only be one owner at a time.
    //   - When the owner goes out of scope, the value will be dropped.

    let x1 = 5;
    let x2 = x1;
    println!("x1 = {}, x2 = {}", x1, x2);

    let s1 = String::from("this is a string");
    let s2 = s1.clone(); // deep copy
    println!("s1 = {}, s2 = {}", s1, s2);

    // Ownership and Functions
    ownership_and_functions();

    // Return Values and Scope
    return_values_and_scope();
}

fn ownership_and_functions() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 3; // x comes into scope

    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("some string: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("some integer: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn return_values_and_scope() {
    let s1 = gives_ownership(); // gives_ownership moves its return value into s1

    println!("s1 = {}", s1);

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("s3 = {}", s3);
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.

// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// takes_and_gives_back takes a String and returns one
fn takes_and_gives_back(some_string: String) -> String {
    // some_string comes into scope
    println!("some string: {}", some_string);
    some_string // some_string is returned and moves out to the calling function
}
