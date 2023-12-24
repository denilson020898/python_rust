use chrono::{DateTime, Local};

use crate::stocks::enums::order_types::OrderType;

use super::stock::Stock;

/// This struct is super awesome
///
/// # Fields
/// * date (Datetime<Local>): the datetime when the order was initially made
/// * stock (Stock): the stock begin involved in the order, refer to Stock for more detail
/// * number (i32): number of stock being involved in this order
/// * order_type (OrderType): Long and Short, assume it whatever you want.
pub struct Order {
    pub date: DateTime<Local>,
    pub stock: Stock,
    pub number: i32,
    pub order_type: OrderType,
}

impl Order {
    pub fn new(stock: Stock, number: i32, order_type: OrderType) -> Order {
        let today: DateTime<Local> = Local::now();
        Order {
            date: today,
            stock,
            number,
            order_type,
        }
    }

    pub fn current_value(&self) -> f32 {
        return self.stock.current_price * self.number as f32;
    }

    pub fn current_profit(&self) -> f32 {
        let current_price: f32 = self.current_value();
        let initial_price: f32 = self.stock.open_price * self.number as f32;
        match self.order_type {
            OrderType::Long => return current_price - initial_price,
            OrderType::Short => return initial_price - current_price,
        }
    }
}
