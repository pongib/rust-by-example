fn main() {
    loop {
        println!("again");
    }
}

// return loop value and assing to variable
fn main() {
    let mut counter = 0;

    let result = loop {
        // assign return value from loop to variable
        counter += 1;

        if counter == 10 {
            break counter * 5; // break with return value
        }
    };

    println!("result is {result}"); // result is 50
}

// label loop
fn main() {
    let mut count = 0;
    // labeled loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // break innermost loop
            }
            if count == 2 {
                break 'counting_up; // break labeled loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

// return loop label with break label return value
fn main() {
    let mut count = 0;
    // labeled loop return value
    let result = 'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // break innermost loop
            }
            if count == 2 {
                // break labeled loop with return value
                break 'counting_up count + 99;
            }
            remaining -= 1;
        }
        count += 1;
    };

    println!("End count = {count}");
    println!("End result = {result}");
}

// while
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// for
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn main() {
    for number in 1..4 {
        // is [1, 2, 3]
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    for number in (1..4).rev() {
        // is reverse
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
