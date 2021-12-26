#[derive(Debug)]
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: i32,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone1@example.com"),
        username: String::from("someone1"),
        active: true,
        sign_in_count: 1,
    };

    // Note that the entire instance must be mutable;
    // Rust doesn’t allow us to mark only certain fields as mutable.
    user1.email = String::from("anotheremail1@example.com");
    println!("{:?}", user1);

    let user2 = build_user(
        String::from("someone2@example.com"),
        String::from("someone2"),
    );
    println!("{:?}", user2);

    let user3 = User {
        email: String::from("someone3@example.com"),
        ..user1
    };
    println!("{:?}", user3);

    // Using Tuple Structs without Named Fields to Create Different Types
    #[derive(Debug)]
    struct Color(i32, i32, i32);

    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:?}", black);
    println!("{:?}", origin);

    // Unit-Like Structs Without Any Fields
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
