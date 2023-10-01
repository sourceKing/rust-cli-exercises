pub fn fn1() -> u32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn1() {
        assert_eq!(1, fn1(), "Testing fn1 returns 1");
    }
}
