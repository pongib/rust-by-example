fn main() {
    // type annotated
    let logical: bool = true;
    // regular annotation
    let a_float: f64 = 1.01;
    // suffix annotation
    let an_integer = 5i64;
    // default
    // f64
    let default_float = 1.23;
    // i32
    let default_integer = 8;
    // inferred type
    let mut inferred_type = 12; // i64 from another line
    inferred_type = 12541211i64;

    // mutate variable value can be change later
    let mut mutable = 22;
    mutable = 9000121;

    // but type can't be changed.
    // This will Error
    // mutable = true;
    // but can overwritten with shadowing
    let mutable = true;

    // compound type
    // array
    let array_int = [123, 456, 789];
    // tuple
    let tuple_like = (100, true, "pong", 'a');
}
