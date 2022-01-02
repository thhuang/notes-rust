fn main() {
    // Creating a New Vector
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // Updating a Vector
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Dropping a Vector Drops Its Elements
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("The third element is {}.", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}.", third),
        None => println!("There is no third element."),
    }

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {}", first);

    let v = vec![100, 200, 300];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 200, 300];
    for i in &mut v {
        println!("{}", i);
        *i += 50;
        println!("{}", i);
    }

    // Using an Enum to Store Multiple Types
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for v in &row {
        println!("{:?}", v);
    }
}
