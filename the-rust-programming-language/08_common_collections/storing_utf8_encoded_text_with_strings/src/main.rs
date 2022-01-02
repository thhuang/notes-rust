use std::io::Bytes;

fn main() {
    // Creating a New String

    let mut s = String::new();

    let data_with_display_trait = "initial content";
    let s = data_with_display_trait.to_string();

    let s = "works on a literal directly".to_string();

    let s = String::from("initial content");

    // Updating a String

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("lo");
    s.push('l');

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    for c in "नमस्ते".bytes() {
        print!("{} ", c);
    }
    println!();

    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();
}
