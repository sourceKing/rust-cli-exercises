pub fn fn3() -> u32 {
    3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn3() {
        assert_eq!(fn3(), 3, "Testing return of fn3 returns 3");
    }
}
