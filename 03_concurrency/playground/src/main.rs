use fib_process::fibonacci_numbers;
use rayon::prelude::*;
use std::any::Any;
use std::thread::JoinHandle;
use std::{thread, time, env};

mod fib_process;

fn simple_thread(seconds: i8, name: &str) -> i8 {
    println!("thread {} is running now!", name);
    let total_seconds = time::Duration::new(seconds as u64, 0);
    thread::sleep(total_seconds);
    println!("thread {} has finished mF!", name);
    return seconds;
}

fn process_thread(thread_result: Result<i8, Box<dyn Any + Send>>, name: &str) {
    match thread_result {
        Ok(res) => {
            println!("the result for {} is {}", res, name);
        }
        Err(err) => {
            if let Some(string) = err.downcast_ref::<String>() {
                println!("the error for {} is : {}", name, string);
            } else {
                println!("the error for {} does not have a message", name);
            }
        }
    }
}

pub fn fibo_recur(n: i32) -> u64 {
    match n {
        0 => panic!("{} is negative, you moron!", n),
        1 | 2 => 1,
        _ => fibo_recur(n - 1) + fibo_recur(n - 2),
    }
}

fn main() {
    let example_closure: fn(&str) = |string_input: &str| {
        println!("{}", string_input);
    };

    example_closure("this is a closure mf");

    let base_rate: f32 = 0.03;
    let calculate_interest = |loan_amount: &f32| {
        return loan_amount * &base_rate;
    };

    println!(
        "the total interest to be paid is: {} mf",
        calculate_interest(&32567.6)
    );

    let now = time::Instant::now();
    let thread_one: JoinHandle<i8> = thread::spawn(|| simple_thread(1, "one"));
    let thread_two: JoinHandle<i8> = thread::spawn(|| simple_thread(1, "two"));
    let thread_three: JoinHandle<i8> = thread::spawn(|| simple_thread(1, "three"));

    // let res_one = thread_one.join().unwrap();
    // let res_two = thread_two.join().unwrap();
    // let res_three = thread_three.join().unwrap();
    // println!("{}{}{}", res_one, res_two, res_three);

    let res_one = thread_one.join();
    let res_two = thread_two.join();
    let res_three = thread_three.join();

    println!("{:?}", now.elapsed());

    process_thread(res_one, "one");
    process_thread(res_two, "two");
    process_thread(res_three, "three");

    let now = time::Instant::now();

    fibo_recur(8);
    fibo_recur(12);
    fibo_recur(12);
    fibo_recur(12);
    fibo_recur(12);
    fibo_recur(20);
    fibo_recur(20);
    fibo_recur(20);
    fibo_recur(20);
    fibo_recur(28);
    fibo_recur(28);
    fibo_recur(28);
    fibo_recur(28);
    fibo_recur(36);

    println!("{:?} has elapsed.", now.elapsed());

    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    let now = time::Instant::now();
    let numbers: Vec<i32> = vec![8, 12, 12, 12, 12, 20, 20, 20, 20, 28, 28, 28, 28, 36];
    let outcomes: Vec<u64> = numbers.into_par_iter().map(|n| fibo_recur(n)).collect();
    println!("{:?}", outcomes);
    println!("{:?} has elapsed with rayon.", now.elapsed());

    // let numbers2: Vec<i32> = vec![
    //     21, 144, 144, 6765, 6765, 6765, 6765, 317811, 317811, 317811, 317811, 14930352, 1836311903,
    //     1836311903, 1836311903,
    // ];
    // let outcomes2: Vec<u64> = numbers2.into_par_iter().map(|n| fibo_recur(n)).collect();
    // println!("{:?}", outcomes2);
    // println!("{:?} has elapsed with rayon.", now.elapsed());

}
