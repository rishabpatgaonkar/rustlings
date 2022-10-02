// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    let answer = square(8) + 5;
    println!("The square of 3 plus 5 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}
