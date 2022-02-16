fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val)
    }

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

#[test]
fn iterator_demonstration() {
    let v = vec![1, 2, 3];
    let mut v_iter = v.iter();

    assert_eq!(v_iter.next(), Some(&1));
    assert_eq!(v_iter.next(), Some(&2));
    assert_eq!(v_iter.next(), Some(&3));
    assert_eq!(v_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v = vec![1, 2, 3];
    let v_iter = v.iter();

    let total: i32 = v_iter.sum();

    assert_eq!(total, 6);
}

// Using Closures that Capture Their Environment

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    return shoes.into_iter().filter(|s| s.size == shoe_size).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

// Creating Our Own Iterators with the Iterator Trait

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            return Some(self.count);
        }
        None
    }
}

#[test]
fn test_counter_iterator() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    let counter = Counter::new();
    let v: Vec<_> = counter.collect();
    assert_eq!(v, vec![1, 2, 3, 4, 5]);
}

#[test]
fn test_other_iterator_methods() {
    let got: u32 = Counter::new()
        .zip(Counter::new().skip(1)) // zip returns None when either of its input iterators return None
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(got, 18);
}
