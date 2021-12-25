fn main() {
    let num: u32 = "42".parse().expect("Not a number.");
    println!("{}", num);

    // Integer Types
    // To explicitly handle the possibility of overflow, you can use these families of methods that the standard library provides on primitive numeric types:
    //   - Wrap in all modes with the wrapping_* methods, such as wrapping_add
    //   - Return the None value if there is overflow with the checked_* methods
    //   - Return the value and a boolean indicating whether there was overflow with the overflowing_* methods
    //   - Saturate at the value’s minimum or maximum values with saturating_* methods

    // Floating-Point Types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
    println!("{} {}", x, y);

    // Numeric Operations
    // addition
    let sum = 5 + 10;
    println!("{}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("{}", difference);
    // multiplication
    let product = 4 * 30;
    println!("{}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);
    let floored = 2 / 3; // Results in 0
    println!("2 / 3 = {}", floored);
    // remainder
    let remainder = 43 % 5;
    println!("{}", remainder);

    // The Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("{} {}", t, f);

    // The Character Type
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{} {} {}", c, z, heart_eyed_cat);

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);
    println!("{:#?}", tup);
    let (x, y, z) = tup; // destructuring
    println!("{} {} {}", x, y, z);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{} {} {}", five_hundred, six_point_four, one);
    let unit_value = (); // unit type and unit value
    println!("{:?}", unit_value);

    // The Array Type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", a);
    let b = [3; 5];
    println!("{:?}", b);
    let first = a[0];
    let second = a[1];
    println!("{} {}", first, second);
}
