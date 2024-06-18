use std::{ops::Not, sync::{Arc, Mutex}, thread};

use actionx::Processor;
use collections::test_iterators;
use database::Database;
use matches::test_matches;
use models::{ random_animal, Animal, Counter, Iterator, Widget };
use orderproducer::OrderProducer;
use time::Duration;

mod models;
mod database;
mod actionx;
mod collections;
mod orderproducer;
mod matches;
mod mutability;

fn main() {
    let animal = random_animal("Janko", 12);
    let noise = animal.make_noise();
    println!("Animal making noise: {} {}", noise, animal.make_noise());

    let start: u16 = 58;
    let mut counter = Counter::new(start);
    println!("Counter value:{:?}", counter.next().unwrap());
    println!("Counter value:{}", counter.next().unwrap());
    println!("Counter value:{}", counter.next().unwrap());

    let text = "fd df 2f";
    let c = last_char(text);
    println!("{:?}", c);

    println!("----");
    println!("W: {:?}", W::new(4,1));
    println!("W: {:?}", W::new(1,4));
    println!("Inside:{:?}", inside());

    let widget = Widget::default();
    println!("Widget: {:?}", widget);

    test_iterators();
    test_matches();


    // let database = Arc::new(Mutex::new(Database::new()));
    
    // let mut order_producer = OrderProducer::new(database.clone());
    // order_producer.start_periodic(3, 2);
    // println!("Started order_producer");

    // let mut order_processor: Processor = Processor::new(database.clone());
    // order_processor.process();

    // thread::sleep(std::time::Duration::from_secs(10));

    // order_processor.process();

    // thread::sleep(std::time::Duration::from_secs(10));

    // order_processor.process();


}

fn inside() -> Option<u16> {
    let mut counter = Counter::new(30);
    let next = counter.next()?;
    println!("Counter value:{:?}", next);
    Some(next)    
}

fn last_char(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn next_int() -> Option<u16> {
    Some(1)
}

#[derive(Debug)]
struct W(String, String);

#[derive(Debug)]
enum SomeError{ 
    FirstError,
    SecondError(String)
}

fn fallible (input: u8) -> Result<String, SomeError> {
    match input {
        0 => Ok(input.to_string()),
        1 => Ok("aha".to_string()),
        2 => Err(SomeError::FirstError),
        _ => Err(SomeError::SecondError(format!("Testing {}", input)))
    }
}

impl W {
    fn new (x: u8, y: u8) -> Result<Self, SomeError> {
        fallible(x).and_then(|x| {
            println!("fallible on first param was ok");
            fallible(y).map(|y| Self(x,y))
        }) 
    }
}

impl W {
    fn new2 (x: u8, y: u8) -> Result<Self, SomeError> {
        fallible(x).and_then(|x| {
            println!("fallible on first param was ok");
            fallible(y).map(|y| Self(x,y))
        }) 
    }
}
