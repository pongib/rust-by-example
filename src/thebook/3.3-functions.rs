fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
// with function
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {x}");

    let x = plus_one(x);

    println!("The value of x shadow is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
