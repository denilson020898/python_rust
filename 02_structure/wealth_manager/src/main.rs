use serde_json::{json, Value};
use stocks::structs::stock::Stock;

mod stocks;

/// Adds two numbers (moved) then returns it.
///
/// # Arguments
/// * one (i32): first number
/// * two (i32): second number
///
/// # Returns
/// (i32): the sum of first and two
///
/// # Usage
/// The function can be used like this way:
///
/// ```rust
/// result: i32 = add_numbers(2, 5);
/// ```
fn add_numbers(one: i32, two: i32) -> i32 {
    return one + two;
}

fn main() {
    let stock: Stock = Stock::new("MonolituhAi", 36.5);
    println!("here is the stock name: {}", stock.name);
    println!("here is the stock current price: {}", stock.current_price);
}
