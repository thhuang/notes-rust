use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        assert_eq!(3, add_one(2));
    }
}
