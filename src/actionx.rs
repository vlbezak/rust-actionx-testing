use std::collections::HashMap;
use std::ptr;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

use crate::database::Database;
use crate::database::DbOrder;
use crate::database::DbOrderStatus;

pub enum ProcessingError {
    DbError(String),
    ProcessError(String),
}

pub struct ActionConfig {
}

impl ActionConfig {

    pub fn new() -> Self {
        ActionConfig {}
    }

    pub fn get_processing_action(&self, action_type_str: &str) -> Result<Box<dyn Action>, ()> {
        let action_type = ActionType::from_str(action_type_str).unwrap();
        match action_type {
            ActionType::SayHello => Ok(Box::new(SayHelloAction{})),
            ActionType::Eat => Ok(Box::new(EatAction {})),
            ActionType::Sleep => Ok(Box::new(SleepAction {})),
        }
    }
}
 

#[derive(Debug,Eq,Hash,PartialEq)]
pub enum ActionType {
    SayHello,
    Eat,
    Sleep
}

impl FromStr for ActionType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hello" => Ok(ActionType::SayHello),
            "eat" => Ok(ActionType::Eat),
            "sleep" => Ok(ActionType::Sleep),
            _ => Err(()),
        }
    }
}

pub trait Action {
    fn process(&self,  order: &mut DbOrder) -> Result<(), ProcessingError>;
}

struct SayHelloAction;
struct EatAction;
struct SleepAction;

impl Action for SayHelloAction {
    fn process(&self, order: &mut DbOrder) -> Result<(), ProcessingError> {
        println!("Processing SayHello {}", order.id);
        Ok(())
    }
}

impl Action for EatAction {
    fn process(&self, order: &mut DbOrder) -> Result<(), ProcessingError> {
        println!("Processing Eat {}", order.id);
        Ok(())
    }
}

impl Action for SleepAction {
    fn process(&self, order: &mut DbOrder) -> Result<(), ProcessingError> {
        println!("Processing Sleep {}", order.id);
        order.status = DbOrderStatus::Success;
        Ok(())
    }
}



pub struct Processor {
    database: Arc<Mutex<Database>>,
    action_config: ActionConfig,
}

impl Processor {
    pub fn new(database: Arc<Mutex<Database>>) -> Processor {
        Processor {
            database,
            action_config: ActionConfig::new(),
        }
    }

    pub fn process(&self) -> Result<(), ProcessingError> {
        println!("--------------------");
        println!("Processor processing");
        let mut binding = self.database.lock().unwrap();
        let orders = binding.get_orders(|order| order.status == DbOrderStatus::New);
        for order in orders {
            let order_action_str = &order.action;
            let action = self.action_config.get_processing_action(order_action_str).unwrap();
            let action = action;
            action.process(order);
        }
        println!("Processor finished processing");
        Ok(())
    }
}
