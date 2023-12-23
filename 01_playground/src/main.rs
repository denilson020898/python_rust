use std::{collections::HashMap, fmt::Display};

fn print(input: &str) {
    println!("{}", input);
}

#[derive(Debug)]
enum Value {
    Str(&'static str),
    Int(i32),
}

fn get_highest<'a, 'b>(first_number: &'a i8, second_number: &'b i8) -> &'a i8 {
    if first_number > second_number {
        return first_number;
    } else {
        return &0;
    }
}

fn process_enum(value: &Value) -> () {
    match value {
        Value::Str(inside_value) => {
            println!("the following value is an str: {}", inside_value);
        }
        Value::Int(inside_value) => {
            println!("the following value is an int: {}", inside_value);
        }
    }
    ()
}

fn check_int_above_threshold(
    threshhold: i32,
    get_result: Option<&Value>,
) -> Result<bool, &'static str> {
    match get_result {
        Some(inside_value) => match inside_value {
            Value::Str(_) => {
                return Err("str value was supplied as opposed to an in which is needed")
            }
            Value::Int(int_value) => {
                if int_value > &threshhold {
                    return Ok(true);
                }
                return Ok(false);
            }
        },
        None => return Err("no value was supplied to be checked"),
    }
}

// A
// |
// |
// |
// |
// |
// B = &A
// |\
// | \
// |  |
// |  |
// |  |
// |  X -> Deallocation
// A

// A
// |
// |
// |
// |
// |
// B = &mut A
// X\
// X \
// X  |
// X  |
// X  |
// X  X -> Deallocation
// A

struct Stock {
    name: String,
    open_price: f32,
    stop_loss: f32,
    take_profit: f32,
    current_price: f32,
}

impl Stock {
    fn new(stock_name: &str, price: f32) -> Self {
        Self {
            name: String::from(stock_name),
            open_price: price,
            stop_loss: 0.0,
            take_profit: 0.0,
            current_price: price,
        }
    }

    fn with_stop_loss(mut self, value: f32) -> Self {
        self.stop_loss = value;
        return self;
    }

    fn with_take_profit(mut self, value: f32) -> Self {
        self.take_profit = value;
        return self;
    }

    fn update_price(&mut self, value: f32) {
        self.current_price = value;
    }
}

trait CanTransfer {
    fn transfer_stock(&self) -> ();

    fn print(&self) -> () {
        println!("a transfer is happening");
    }
}

impl CanTransfer for Stock {
    fn transfer_stock(&self) -> () {
        println!(
            "the stock {} is being transferred for {} idr",
            self.name, self.current_price
        );
    }
}

fn process_transfer(stock: impl CanTransfer) {
    stock.print();
    stock.transfer_stock();
}

struct Coordinate<T> {
    x: T,
    y: T,
}

macro_rules! capitalize {
    ($a: expr) => {
        let mut v: Vec<char> = $a.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        $a = v.into_iter().collect();
    };
}

#[derive(Clone, Copy)]
struct Coordinate2 {
    x: i8,
    y: i8,
}

fn print2(point: Coordinate2) {
    println!("{} {}", point.x, point.y);
}

fn main() {
    let test_string = &"Hello, world!";
    print(test_string);

    let mut result = 1.0 + 2.2;
    result = result + 3.3;
    print(&result.to_string());

    let mut map = HashMap::new();

    map.insert("one", Value::Str("1"));
    map.insert("two", Value::Int(2));

    println!("{:#?}", map);

    for (key, value) in &map {
        process_enum(value);
    }

    match map.get("test") {
        Some(value) => process_enum(value),
        None => println!("I got nothing bro"),
    }
    // let outcome: Option<&Value> = map.get("test");
    // println!("outcome passed");
    // let another_outcome: &Value = map.get("test").unwrap();
    // println!("another outcome passed");

    let result: Option<&Value> = map.get("two");
    let above_threshold: bool = check_int_above_threshold(1, result).unwrap();
    println!("it is {} that the threshold is breached.", above_threshold);

    let above_threshold: bool = check_int_above_threshold(2, result).unwrap();
    println!("it is {} that the threshold is breached.", above_threshold);

    // let result : Option<&Value> = map.get("one");
    // let above_threshold: bool = check_int_above_threshold(1, result).expect("an error happened");
    // println!("it is {} that the threshold is breached.", above_threshold);

    let one: String = String::from("one");
    let two: String = one.to_owned() + " two";
    println!("{}", two);
    println!("{}", one);

    let one: i8 = 1;
    {
        let two: i8 = 2;
        let outcome: &i8 = get_highest(&one, &two);
        println!("{}", outcome);
    }

    let stock: Stock = Stock::new("monolith ai", 95.0).with_stop_loss(55.0);
    let stock_two: Stock = Stock::new("monolith ai", 95.0).with_stop_loss(55.0);
    let mut stock_three: Stock = Stock::new("monolith ai", 95.0)
        .with_take_profit(101.1)
        .with_stop_loss(55.0);

    println!("{}", stock_three.current_price);
    stock_three.update_price(123.456);
    println!("{}", stock_three.current_price);

    stock_three.print();
    stock_three.transfer_stock();

    stock_three.update_price(123456.123);
    process_transfer(stock_three);

    let one = Coordinate { x: 50, y: 50 };

    let two = Coordinate { x: 500, y: 500 };

    let three = Coordinate { x: 5.6, y: 6.5 };

    let mut x = String::from("test");
    capitalize!(x);
    println!("{}", x);

    let test = Coordinate2{x:1,y:2};
    print2(test);
    println!("{}", test.x);
}
