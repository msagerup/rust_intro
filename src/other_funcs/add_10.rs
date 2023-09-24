///Function: add_10
///
/// # Args (num: 32)
///
/// # Example
/// ```
/// let x = 5
/// add_10(x)
/// ```

pub fn add_10(num: u32) -> u32 {
    num + 10
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn adds_10_test() {
        let x: u32 = 100;
        let y: u32 = add_10(x);
        println!("y is from test: {}", y);
        assert_eq!(y, 110);
    }
}
