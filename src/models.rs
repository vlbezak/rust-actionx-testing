use rand::prelude::*;

struct BaseData {
    name: String,
    age: u8,
}

pub struct Dog {
    name: String,
    age: u8,
}

pub struct Cat {
    name: String,
    age: u8,
}

pub trait Animal {
    fn make_noise(&self) -> &'static str;
}

pub trait Named {
    fn name(&self) -> String {
        "name".to_string()
    }
}

pub fn random_animal(name: &str, age: u8) -> Box<dyn Animal> {
    let mut rng = rand::thread_rng();
    let rand_number: f64 = rng.gen();
    if rand_number < 0.5 {
        Box::new(Cat {
            name: name.to_string(),
            age,
        })
    } else {
        Box::new(Dog { name: name.to_string(), age })
    }
}

impl Animal for Cat {
    fn make_noise(&self) -> &'static str {
        "meeeeow!"
    }
}

impl Named for Dog {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Animal for Dog {
    fn make_noise(&self) -> &'static str {
        "wooooof!"
    }
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

pub struct Counter {
    i: u16
}

impl Iterator for Counter {
    type Item = u16;

    fn next(&mut self) -> Option<Self::Item> {
        self.i += 1;
        Some(self.i)
    }
}

impl Counter {
    pub fn new(start: u16) -> Self {
        Counter {i: start}
    }
}

#[derive(Debug)]
pub struct Data {
}

impl Data {
    pub fn crunch(&self) -> i32 { 42 }
}

#[derive(Debug)]
pub struct Widget(Option<Data>);

impl Widget{
    pub fn data_a(&self) -> &Option<Data> {
        &self.0
    }
    
    pub fn data_b(&self) ->Option<&Data> {
        self.0.as_ref()
    }
}

impl Default for Widget {
    fn default() -> Self {
        Widget ( Some(Data{}))
    }
}

