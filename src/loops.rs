pub fn loops() {
    println!("--- Loops ---");
    ///////////////// loops in rust
    // loop, while, for
    let mut count = 0;
    // loop: infinite loop until break
    // loop {
    //     count += 1;
    //     if count == 5 {
    //         println!("Count reached 5, breaking the loop.");
    //         break;
    //     }
    // }
    // Loop labels help differ between multiple loops
    'outer: for i in 1..4 {
        // i takes values 1,2,3
        for j in 1..4 {
            // j takes values 1,2,3
            if i * j == 4 {
                println!("Breaking out of outer loop when i * j == 4");
                break 'outer;
            }
            println!("i: {}, j: {}", i, j);
        }
    }
}
