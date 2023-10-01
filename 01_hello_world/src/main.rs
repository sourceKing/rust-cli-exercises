//!   The simplest of simple for Rust programs!

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = say_hello_world();
    print!("{}", s);
    Ok(())
}

fn say_hello_world() -> String {
    "Hello, World!".into()
}

#[cfg(test)]
mod tests {
    // import from outer scope
    use super::*;

    #[test]
    fn print_hello_world() {
        assert_eq!(
            "Hello, World!".to_string(),
            say_hello_world(),
            "Testing the print of hello, world"
        );
    }
}
