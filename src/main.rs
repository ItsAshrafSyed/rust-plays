mod compound;
mod enums;
mod loops;
mod primitive;
mod shadowing;
mod string_slice;
mod structs;

fn main() {
    primitive::primitive_types();
    compound::compound_types();
    string_slice::string_and_slice();

    ///////////////// ownership, borrowing in rust
    //// ownership:
    // 1.each value in Rust has a variable that’s called its owner.
    // 2.there can only be one owner at a time.
    // 3.when the owner goes out of scope, the value will be dropped.
    //// borrowing & references:
    // 1.references allow you to refer to some value without taking ownership of it.
    // 2.you can have either one mutable reference or any number of immutable references to a value.

    // /////////////// struct and impl in rust
    let mut account = structs::BankAccount {
        owner: String::from("Jhon"),
        balance: 100.00,
    };
    account.check_balance();
    account.withdraw(20.0);
    account.check_balance();

    // - evary variable by default is immutable in rust unless we use mut keyword
    // - constants are always immutable in rust meaning we cannot use mut keyword with constants, must declare the type of constant & use all uppercase with underscores. you can declare constants in any scope including global scope
    // const MAX_POINTS: u32 = 100_000;

    shadowing::shadowin();
    loops::loops();
    enums::enums();

    ///////////////// error handling in rust
    // 1. Options<T> enum is used when a value could be something or nothing
    // 2. Result<T, E> enum is used for functions that can return a value or an error
    // 3. panic! macro is used when a program encounters an unrecoverable error and needs to crash

    // enum Option<T> {
    //     Some(T), // Represents a value
    //     None,    // Represents the absence of a value
    // }
    //example
    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }
    let result = divide(10.0, 2.0);
    match result {
        // this is called pattern matching and is similar to switch case in other languages. Here it is used to handle the Option enum returned by divide function
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero"),
    }
    // enum Result<T, E> {
    //     Ok(T),  // Represents a successful value
    //     Err(E), // Represents an error
    // }
    //example
    fn multiply(a: i32, b: i32) -> Result<i32, String> {
        if a == 0 || b == 0 {
            Err(String::from("Cannot multiply by zero"))
        } else {
            Ok(a * b)
        }
    }
    let result = multiply(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {}", e), // here e is the error message returned by multiply function
    }
}
