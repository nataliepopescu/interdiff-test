fn greet(name: &str) -> String {
    "Hello, ".to_string() + name
}

fn farewell(name: &str) -> String {
    "Goodbye, ".to_string() + name
}

fn main() {
    println!("{}", greet("world"));
    println!("{}", farewell("world"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greets() {
        assert!(greet("world").starts_with("Hello, world"));
    }
}

#[test]
fn farewells() {
    assert!(farewell("world").starts_with("Goodbye, world"));
}
