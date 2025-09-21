pub fn shadowin() {
    println!("--- Shadowing ---");
    ///////////////// shadowing in rust
    let x = 5;
    let x = x + 1; // shadowing the previous x
    {
        let x = x * 2; // shadowing the previous x in inner scope
        println!("Inner scope x: {}", x); // prints 12
    }
    println!("Outer scope x: {}", x); // prints 6

    // shadowing is not the same as mutability in rust beccaause we are not reassigning the value of x we are creating a new variable with the same name everytime we use let keyword
}
