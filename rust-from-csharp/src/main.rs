fn main() {
    section2_logic_flow();
}

fn section1_scratch() {
    println!("Hello Rust, from the C# world!");

    let name = String::from("Rust");
    let greeting = format!("Hello, {}!", name);
    println!("{}", greeting);

    let mut number = 1;

    number = number + 2;

    println!("The number is {}", number);
}

fn section2_variables_mutability() {
    // Note: All of these are immutable by default, unlike in C# and other languages
    let test = "";

    let number = 1;
    let num_64: i64 = 1;

    let is_true = true;

    // To make something mutable, use the mut keyword
    let mut mutable_number = 1;

    mutable_number = mutable_number + 2;

    println!("The number is {}", mutable_number);

    // Null is not a thing in Rust, so the following line would not compile
    // let some_value = null;

    // Instead, we use the Option type, like in F#
    let some_value_as_none: Option<i32> = None;
    let some_value_with_value: Option<i32> = Some(1);
}

fn section2_functions_macros() {
    let result = double(2);

    println!("The result is {}", result);

    // Note the ! at the end of println. This is because it is a macro, not a function.
    // Macros are expanded at compile time, not runtime. This allows us to use a dynamic
    // number of arguments.
    println!("The result is {}", double(3));
}

// Note: Type comes after the variable name, unlike in C#.
//       We also put the return type after the function name using ->.
fn double(x: i32) -> i32 {
    // Note: Implicit return, like in F#
    x * 2
}

fn section2_logic_flow() {
    let result = half(30);

    match result {
        Some(x) if x >= 10 => println!("Very big number here!"),
        Some(x) => println!("The result is {}", x),
        // Note: We could use None here, but we can also use _ to match anything
        _ => println!("Failed to half"),
    }
}

fn half(num: i32) -> Option<i32> {
    if num == 0 {
        return None;
    }
    if num % 2 == 0 {
        Some(num / 2)
    } else {
        None
    }
}

#[test]
fn test_case() {
    assert_eq!(1 + 2, 3);
}
