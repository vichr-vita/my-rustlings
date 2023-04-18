// variables2.rs
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a hint.

fn main() {
    let x: Option<i64> = None;
    if x == Some(10) {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
