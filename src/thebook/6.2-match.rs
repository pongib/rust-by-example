// match with enum inside enum
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

// get value inside option
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    fn multiply_ten(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => {
                println!("i is {} and * 10 = {}", i, i * 10);
                Some(i * 10)
            }
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let ten = Some(10);
    let hundrend = multiply_ten(ten);
}

// Non-Exhaustive pattern

fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
    }

    // If not cover all variant it will error
    // called non-exhaustive patterns
    fn handle_coin(coin: Coin) -> i32 {
        match coin {
            Coin::Penny => 3,
        }
    }
}

// Catch-all
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

// Catch all but don't want to use any left value

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // not use any left variable
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}

// Catch-all but do nothing
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // just do nothing
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}

// quiz solution
enum Location {
    Point(i32),
    Range(i32, i32),
}

fn main() {
    fn print_range_max(loc: &Location) {
        // print the second field of Range, if loc is a Range
        if let Location::Range(_, second) = loc {
            println!("{}", second);
        }
    }

    let location = Location::Range(10, 20);
    print_range_max(&location)
}
