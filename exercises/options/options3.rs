// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

struct Point {
    x: i32,
    y: i32,
}

// Option<T> is an enum in Rust that can either be Some(T) or None.
// It's used to represent a value that may or may not be present.

fn main() {
    // Here, y is an Option that contains a Point. Since it's wrapped in Some,
    // it represents a present value (not necessarily 'true' in a boolean sense).
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    // In this match statement, we're checking if y is Some with a value or None.
    // The 'ref' keyword is used to borrow the value inside the Option,
    // allowing access to p's fields without taking ownership of the value.
    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    // This line demonstrates that y is still valid (not moved or consumed)
    // after the match block. If 'ref' wasn't used, this would result in a
    // compile-time error because y would be moved into the match.
    y;
}
