fn main() {
    println!("A");
}

pub fn a() {
    println!("Hello, this is A");
}

#[cfg(feature="foo")] // Commenting this line allow B's test to build
pub fn foo(x: u32) -> u32 {
    x + 1
}