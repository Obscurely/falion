pub mod search;

pub fn add(a: u128, b: u128) -> u128 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
