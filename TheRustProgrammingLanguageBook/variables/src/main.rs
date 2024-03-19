fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    // Constants
    const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Floating-point types
    let x 2.0; // f64

    let y: f32 = 3.0; // f32
    
    // Numeric Opertations
    // addition
    let sum = 5 + 10;

    // subtraccion
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncate = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f: bool = false; // with explicit type annotation
    
    // The Character Type
    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!(The value of y is: {y});

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // The Array Type
    let a = [1, 2, 3, 4, 5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

}
