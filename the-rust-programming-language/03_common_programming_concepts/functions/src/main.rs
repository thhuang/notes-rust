fn main() {
    println!("Hello, world!");

    another_function(5);

    let y = {
        let x = 3;
        x + 5
    };

    println!("y = {}", y);

    let y = add_one(y);
    println!("y after add_one() is {}", y)
}

fn another_function(num: i32) {
    println!("The number is {}", num);
}

fn add_one(x: i32) -> i32 {
    x + 1
}
