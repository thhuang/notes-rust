fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        assert_eq!(4, add_two(2));
    }
}
