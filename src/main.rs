fn main() {
    println!("Hello, world!");
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

    // String and &str (slice string)
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

    let numbers: [i32; 3] = [1, 2, 3];
    let strings: [&str; 3] = ["one", "two", "three"];
    let floats: [f32; 3] = [1.1, 2.2, 3.3];
    let bools: [bool; 3] = [true, false, true];
    let chars: [char; 3] = ['a', 'b', 'c'];
    let tuples: [(i32, &str); 3] = [(1, "one"), (2, "two"), (3, "three")];
    let arrays: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let books: &[String; 3] = &["one".to_string(), "two".to_string(), "three".to_string()];
    let cold: String = "cold".to_string();

    let mut account = BankAccout {
        owner: "John".to_string(),
        balance: 100.0,
    };

    account.check_balance();
    account.withdraw(50.0);
    account.check_balance();

    //struct

    struct BankAccout {
        owner: String,
        balance: f32,
    }

    impl BankAccout {
        fn withdraw(&mut self, amount: f32) {
            self.balance -= amount;
            println!("Withdrew: {}", amount);
        }

        fn check_balance(&self) {
            println!("Balance: {}", self.balance);
        }
    }

    println!("{:?}", numbers);
    println!("{:?}", strings);
    println!("{:?}", floats);
    println!("{:?}", bools);
    println!("{:?}", chars);
    println!("{:?}", tuples);
    println!("{:?}", arrays);
    println!("{:?}", books);
}
