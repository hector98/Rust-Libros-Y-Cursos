fn main() {
    let s = "hello"; // s is not valid here, it's not yet declare
                     // s is valid from this point forward
    // do stuff with s
    // this scope is now over,  and s is no longer valid

    // The string type
    let s = String::from("hello");

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print 'hello, world!'

    // Variables and Data interacting with move
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    // Variables and Data interacting with clone
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Ownership and Functions
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function,
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
                   // use x afterward
               
} // Here, x goes out of scope, then s, But because s's value moved, nothing
  // special happens.

// Ownership and Functions
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
