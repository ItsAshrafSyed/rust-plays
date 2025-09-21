pub fn primitive_types() {
    println!("--- Primitive types ---");
    ///////////////// primitive types in rust
    // integer, float, boolean, char
    // integer: i8, i16, i32, i64, i128
    let i32_var: i32 = -42; // range is -2^31 to 2^31-1
    // unsigned integer: u8, u16, u32, u64, u128
    let u32_var: u32 = 42; // range is 0 to 2^32-1
    // float: f32, f64
    let f64_var: f64 = 3.14;
    // boolean: bool
    let bool_var: bool = true;
    // char: char (4 bytes, Unicode Scalar Value)
    let char_var: char = 'a';
    println!(
        "i32: {}, u32: {}, f64: {}, bool: {}, char: {}",
        i32_var, u32_var, f64_var, bool_var, char_var
    );
}
