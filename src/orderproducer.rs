use std::sync::{ Arc, Mutex };
use std::thread::{ self, JoinHandle };
use time::OffsetDateTime;
use std::time::Duration;

use crate::database::{ self, Database, DbOrder };

struct Scheduling {
    period_seconds: u64,
    pervious_time: Option<OffsetDateTime>,
}

pub struct OrderProducer {
    database: Arc<Mutex<Database>>,
    running: bool,
}

impl OrderProducer {
    pub fn new(database: Arc<Mutex<Database>>) -> OrderProducer {
        OrderProducer {
            database,
            running: false,
        }
    }

    pub fn start_periodic(&mut self, period_seconds: u64, orders_to_produce: u16) {
        if self.running {
            println!("Already running");
            return;
        }

        let mut scheduling = Scheduling {
            period_seconds,
            pervious_time: None,
        };

        let database = self.database.clone();

        let thread_handle = thread::spawn(move || {
            loop {
                scheduling.pervious_time = Some(OffsetDateTime::now_utc());
                perform_iteration(&database, orders_to_produce);
                //:TODO - vyratajme cas ktory nam ostava a sleepnime podla toho
                //let sleep = self.scheduling.unwrap().pervious_time.unwrap().checked_add(Duration::seconds(period_seconds));
                let sleep = Duration::from_secs(period_seconds);
                println!("Producer sleeping for {:?} seconds", sleep);
                thread::sleep(sleep);
            }
        });
    }

    pub fn stop(&mut self) {
        self.running = false;
    }
}

fn perform_iteration(database: &Arc<Mutex<Database>>, orders_to_produce: u16) {
    //TODO - niektore veci nemame vyplnene
    println!("Inserting DbOrder");
    let order = DbOrder::new_minimal("Testing", "sleep");
    let mut db = database.lock().unwrap();
    let new_order = db.insert(order);
    match new_order {
        Ok(ord) => println!("Inserted order:{}", ord.id),
        Err(err) => panic!("Failed to insert order:{:?}", err),
    }
}
