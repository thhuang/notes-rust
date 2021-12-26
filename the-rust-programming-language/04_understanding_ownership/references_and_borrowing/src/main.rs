fn main() {
    let s1 = String::from("this is a string");
    let l = calculate_length(&s1);
    println!("len({}) = {}", s1, l);

    let mut s2 = String::from("hello");
    println!("s2 before change(): {}", s2);
    change(&mut s2);
    println!("s2 after change(): {}", s2);

    // A data race is similar to a race condition and happens when these three behaviors occur:
    //   1. Two or more pointers access the same data at the same time.
    //   2. At least one of the pointers is being used to write to the data.
    //   3. There’s no mechanism being used to synchronize access to the data.

    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 = {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let r2 = &mut s;
    println!("r2 = {}", r2);

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    // let reference_to_nothing = dangle();
    let s = no_dangle();
    println!("{}", s);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     let s = String::from("a string");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("a string");

    s
}
