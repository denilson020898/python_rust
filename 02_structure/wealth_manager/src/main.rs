use serde_json::{json, Value};
use stocks::structs::stock::Stock;

use crate::stocks::{
    close_order, enums::order_types::OrderType, open_order, structs::order::Order,
};

mod stocks;

use rand::prelude::*;
use std::env;
use std::str::FromStr;

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
    println!("hello stonks");

    let args: Vec<String> = env::args().collect();
    let action: &String = &args[1];
    let name: &String = &args[2];
    let amount: i32 = i32::from_str(&args[3]).unwrap();
    let price: f32 = f32::from_str(&args[4]).unwrap();

    let mut new_order: Order =
        open_order(amount, OrderType::Long, &name.as_str(), price, None, None);

    match action.as_str() {
        "buy" => {
            println!(
                "the value of your investmentt is: {}",
                new_order.current_value()
            );
        }
        "sell" => {
            let mut rng = rand::thread_rng();
            let new_price_ref: f32 = rng.gen();
            let new_price: f32 = new_price_ref * 100 as f32;
            new_order.stock.update_price(new_price);
            let sale_profit: f32 = close_order(new_order);
            println!("here is the profit: {}, be grateful!", sale_profit);
        }
        _ => {
            panic!("Only 'buy' and 'sell' actions are available, you stinky!")
        }
    }

    // println!("the current price is: {}", &new_order.current_value());
    // println!("the current profit is: {}", &new_order.current_profit());
    // new_order.stock.update_price(43.1);
    // println!("the current price is: {}", &new_order.current_value());
    // println!("the current profit is: {}", &new_order.current_profit());
    // new_order.stock.update_price(82.7);
    // println!("the current price is: {}", &new_order.current_value());
    // println!("the current profit is: {}", &new_order.current_profit());
    // let profit: f32 = close_order(new_order);
    // println!("we made {} profit boi", profit);
}
