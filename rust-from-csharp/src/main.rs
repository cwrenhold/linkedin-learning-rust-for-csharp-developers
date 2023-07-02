fn main() {
    println!("Hello Rust, from the C# world!");

    let name = String::from("Rust");
    let greeting = format!("Hello, {}!", name);
    println!("{}", greeting);

    let mut number = 1;

    number = number + 2;

    println!("The number is {}", number);
}

#[test]
fn test_case() {
    assert_eq!(1 + 2, 3);
}
