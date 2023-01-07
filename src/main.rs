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
