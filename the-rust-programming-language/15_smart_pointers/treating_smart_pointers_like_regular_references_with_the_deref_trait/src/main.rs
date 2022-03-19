use std::ops::Deref;

fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(x, 5);
    assert_eq!(*y, 5);

    let z = MyBox::new(x);
    assert_eq!(*z, 5); // *z => *(z.deref())

    // Deref Coercions
    let name = "Rust";
    hello(&name);
    let name = MyBox::new("Rust");
    hello(&name);
    let name = MyBox::new(String::from("Rust"));
    hello(&(*name)[..]);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
