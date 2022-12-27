fn main() {
    let number = 3;

    if number < 5 {
        println!("condition is true");
    } else {
        println!("Condition was false");
    }
}

// if else if condition
fn main() {
    let number = 11;

    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("not divisible by 2, 3, 4");
    }
}

// if, expression, in let statement

fn main() {
    let condition = true;
    // if expression in let statement
    let number = if condition { 5 } else { 6 };

    println!("Number is {number}");
}

// assign later in condition
fn main() {
    let x;
    if true {
        x = 1;
    } else {
        x = 2;
    }

    println!("x is {x}");
}
