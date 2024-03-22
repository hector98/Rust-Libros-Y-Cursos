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
}
