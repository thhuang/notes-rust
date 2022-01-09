fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &v in list {
        if v > largest {
            largest = v;
        }
    }
    return largest;
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        return &self.x;
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct PointTwoTypes<T, U> {
    x: T,
    y: U,
}

impl<T, U> PointTwoTypes<T, U> {
    fn mixup<V, W>(self, other: PointTwoTypes<V, W>) -> PointTwoTypes<T, W> {
        PointTwoTypes {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p = Point { x: 5, y: 4 };
    println!("p.x() = {}", p.x());

    let p2: Point<f32> = Point { x: 1.1, y: 2.2 };
    println!(
        "p2 ({}, {}) distance from origin: {}",
        p2.x(),
        p2.y,
        p2.distance_from_origin()
    );

    let both_integer = PointTwoTypes { x: 5, y: 10 };
    let both_float = PointTwoTypes { x: 1.0, y: 4.0 };
    let integer_and_float = PointTwoTypes { x: 5, y: 4.0 };

    let p1 = PointTwoTypes { x: 5, y: 10.4 };
    let p2 = PointTwoTypes { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
