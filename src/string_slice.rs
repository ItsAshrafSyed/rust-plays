pub fn string_and_slice() {
    println!("--- String and &str (slice string) ---");
    ///////////////////// String and &str (slice string)
    // String: growable, mutable, owned, allocated on heap. (any data type by default is immutable)
    let mut string_var: String = String::from("Hello, Rust!"); // String::from is used to create a String from a string literal as "Hello, Rust!" is of type &str and put it on heap memory it is equivalent to let string_var = "Hello, Rust!".to_string();
    println!("Computer says {}", string_var);
    // but cuz it is mutable we can change it by having mut keyword in declaration
    string_var.push_str(" Welcome to Rust programming.");
    println!("Computer says {}", string_var);
    // &str: immutable, fixed-length, borrowed string slice, usually in stack
    let string: String = String::from("Hello, Rustaceans!");
    let str_slice: &str = &string; // borrow a slice of the String
    println!("String slice: {}", str_slice);
    let str_slice2: &str = &str_slice[7..11];
    println!("String slice of selected indexs: {}", str_slice2); // slicing the string slice to get "Rust"
}
