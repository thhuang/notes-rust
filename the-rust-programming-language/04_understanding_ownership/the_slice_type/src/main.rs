fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    let s = String::from("hello");

    let slice = &s[0..2];
    println!("{}", slice);
    let slice = &s[..2];
    println!("{}", slice);

    let slice = &s[3..s.len()];
    println!("{}", slice);
    let slice = &s[3..];
    println!("{}", slice);

    let slice = &s[0..s.len()];
    println!("{}", slice);
    let slice = &s[..];
    println!("{}", slice);

    let mut s = String::from("hello world");
    let w = first_word(s.as_str());
    s.clear();
    println!("The first word is \"{}\"", w);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[2..4];
    assert_eq!(slice, &[3, 4]);
}

fn first_word(s: &str) -> String {
    for (i, b) in s.bytes().enumerate() {
        if b == b' ' {
            return String::from(&s[..i]);
        }
    }
    return String::from(s);
}
