use crate::mod1;

pub fn fn2() -> u32 {
    mod1::fn1() * 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn2_return() {
        assert_eq!(2, fn2(), "Testing if fn2 returns 2")
    }
}
