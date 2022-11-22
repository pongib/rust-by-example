fn main() {
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    // boolean logic
    println!(
        "true AND false is {}, true OR false is {}",
        true && false,
        true || false
    );

    println!("NOT true is {}", !true);

    // bitwise operations
    println!("1011 AND 1101 is {:02b}", 0b1011u32 & 0b1101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 is {:b}, in decimal {}", 0x80u32, 0x80u32);
    println!(
        "0x80 >> 2 is 0x{:x} or in binary {:b}",
        0x80u32 >> 2,
        0x80u32 >> 2
    );

    // underscore for readability
    println!("one billion is written as {}", 1_000_000_000u32);
}
