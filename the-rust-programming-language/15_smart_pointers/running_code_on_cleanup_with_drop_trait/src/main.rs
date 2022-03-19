use std::fmt::Debug;

fn main() {
    let a = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let b = CustomSmartPointer { data: 123 };

    println!("CustomSmartPointers created");

    let c = CustomSmartPointer { data: String::from("some data") };
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
} // Variables are dropped in the reverse order of their creation

struct CustomSmartPointer<T: Debug> {
    data: T,
}

// The Drop trait is included in the prelude
impl<T: Debug> Drop for CustomSmartPointer<T> {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{:?}`!", self.data);
    }
}
