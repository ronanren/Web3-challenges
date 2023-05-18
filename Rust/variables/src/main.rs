const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Variables and Mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // Data Types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    let sum = 5 + 10; // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4 * 30; // multiplication
    let quotient = 56.7 / 32.2; // division
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5; // remainder
    let t = true;
    let f: bool = false; // with explicit type annotation
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    let last_elem = tup.2;
    println!("The value of z is: {last_elem}");
    let a: [i32; 3] = [-1, 2, -3];
    let a = a[0];
    println!("The first value of the array: {a}");

    // Functions
    another_function();
}

fn another_function() {
    println!("Another function");
}
