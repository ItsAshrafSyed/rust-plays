pub fn compound_types() {
    println!("--- Compound types ---");
    ///////////////// compound types in rust
    // tuple, array, slices ,strings (slice string)
    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr); // prints array in debug format cuz we add :?
    let fruits: [&str; 3] = ["apple", "banana", "cherry"];
    println!("Fruits array: {:?}", fruits);
    // Tuple
    let tup: (i32, f64, &str) = (42, 3.14, "hello");
    println!("Tuple: {:?}", tup);
    let (x, y, z) = tup; // destructuring
    println!("Destructured Tuple: x={}, y={}, z={}", x, y, z);
    // Slices : dynamically sized view into a contiguous sequence (array or vector)
    let number_slice: &[i32] = &[2, 3, 7, 8];
    println!("Number slice: {:?}", number_slice);
    let fruit_slice: &[&str] = &["mango", "orange", "grape"];
    println!("Fruit slice: {:?}", fruit_slice);
}
