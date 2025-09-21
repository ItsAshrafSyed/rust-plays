mod compound;
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
}
