pub fn add_five(num: u32) -> u32 {
    num + 5
}

pub fn subtract_one(num: u32) -> u32 {
    num - 1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn adds_five_test() {
        let x: u32 = 100;
        let y: u32 = add_five(x);
        println!("y is from test: {}", y);
        assert_eq!(y, 105);
    }

    #[test]
    fn subtracts_one_test() {
        let x: u32 = 100;
        let y: u32 = subtract_one(x);
        println!("y is from test: {}", y);
        assert_eq!(y, 99);
    }
}
