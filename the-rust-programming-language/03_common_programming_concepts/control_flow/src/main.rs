fn main() {
    // if Expressions
    let number = 3;
    if number < 5 {
        println!("The number is less than 5");
    } else {
        println!("The number is greater than or equal to 5");
    }

    // Using if in a let Statement
    let condition = true;
    let x = if condition { 3 } else { 5 };
    println!("x = {}", x);

    // Repetition with Loops
    let mut count = 0;
    let x: i32 = 'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'counting_up count * remaining;
            }
            remaining -= 1;
        }
        count += 1;
    };
    println!("End count = {}", count);
    println!("Loop returns {}", x);

    // Conditional Loops with while
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    for (i, v) in a.iter().enumerate() {
        println!("a[{}] = {}", i, v);
    }
    for number in (1..=10).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
