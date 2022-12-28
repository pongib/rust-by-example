struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // move user1 string
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    // not move user1 just copy
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("someusername456"),
        ..user1
    };

    println!("{} {}", user2.email, user2.username);

    println!("{} {}", user1.email, user1.username);
}

// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 1, 2);
    let origin = Point(0, 0, 0);

    let new_point = take_color(black);
}

fn take_color(color: Color) -> Point {
    Point(color.0, color.1, color.2)
}

// Unit-Like Struct
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}

// mutable borrow on struct
// create reference is called borrow
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = &mut p.x;
    let y = &mut p.y;
    let y = &mut p.x; // will error borrow more than once

    *x += 1;
    *y += 1;
    println!("{} {}", p.x, p.y);
}
