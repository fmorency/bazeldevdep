use A::a;

fn main() {
    a();
    b();
}

fn b() {
    println!("Hello, this is B");
}

#[cfg(test)]
mod tests {
    use A::foo;
    #[test]
    fn maybe_foo() {
        assert_eq!(foo(42), 43);
    }
}