// Using a semicolon after mod front_of_house rather than using a block
// tells Rust to load the contents of the module from another file with
// the same name as the module.
mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

// Specifying the parent module when calling the function
// makes it clear that the function isn’t locally defined
// while still minimizing repetition of the full path.
use crate::front_of_house::hosting;

// When bringing in structs, enums, and other items with use,
// it’s idiomatic to specify the full path.
use std::collections::HashMap;

pub fn eat_at_restaurant() {
    // Our preference is to specify absolute paths because it’s more likely
    // to move code definitions and item calls independently of each other.

    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // `hosting` is brought into the scope with `use crate::front_of_house::hosting`
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    let mut map = HashMap::new();
    map.insert(1, 2);
}

fn serve_order() {}

// The example shows how to bring two Result types into scope that have
// the same name but different parent modules and how to refer to them.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    fmt::Result::Ok(())
}

fn function2() -> io::Result<()> {
    // --snip--
    io::Result::Ok(())
}

// Providing New Names with the as Keyword
use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    // --snip--
    Result::Ok(())
}

fn function4() -> IoResult<()> {
    // --snip--
    IoResult::Ok(())
}

// Re-exporting Names with `pub use`
// To enable the code that calls our code to refer to that name as if
// it had been defined in that code’s scope, we can combine `pub` and `use`.
pub use crate::front_of_house::hosting2;

// Using Nested Paths to Clean Up Large use Lists
use rand::*;
use std::collections::{BTreeMap, BinaryHeap, LinkedList};
use std::{cmp::Ordering, i32};
