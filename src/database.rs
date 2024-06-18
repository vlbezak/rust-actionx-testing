use time::OffsetDateTime;
pub struct Database {
    db_order_seq: DbOrderSeq,
    db_orders: Vec<DbOrder>,
}

#[derive(Debug,Clone,PartialEq)]
pub enum DbOrderStatus {
    New,
    Processing,
    Success,
    Error(String),
}

#[derive(Debug,Clone,PartialEq)]
pub struct DbOrder {
    pub id: i64,
    pub name: String,
    pub created_time: OffsetDateTime,
    pub process_time: Option<OffsetDateTime>,
    pub process_start_time: Option<OffsetDateTime>,
    pub process_end_time: Option<OffsetDateTime>,
    pub action: String,
    pub retry_count: u16,
    pub status: DbOrderStatus,
}

impl DbOrder {
    pub fn new_minimal(name: &str, action: &str) -> DbOrder {
        DbOrder {
            id: -1,
            name: name.to_string(),
            action: action.to_string(),
            created_time: OffsetDateTime::now_utc(),
            process_time: None,
            process_start_time: None,
            process_end_time: None,
            retry_count: 0,
            status: DbOrderStatus::New, 

        }
    }
}

pub trait Seq {
    fn current(&self) -> i64;
    fn next(&mut self) -> i64;
}

pub struct DbOrderSeq {
    index: i64,
}

impl DbOrderSeq {
    pub fn new(start: i64) -> DbOrderSeq {
        DbOrderSeq { index: start }
    }
}

#[derive(Debug)]
pub enum DbError {
    NotFound,
    Duplicate,
}

impl Seq for DbOrderSeq {
    fn current(&self) -> i64 {
        self.index
    }
    fn next(&mut self) -> i64 {
        self.index += 1;
        self.index
    }
}

impl Database {
    pub fn new() -> Database {
        Database {
            db_order_seq: DbOrderSeq::new(1),
            db_orders: Vec::new(),
        }
    }

    pub fn insert(&mut self, mut order: DbOrder) -> Result<&DbOrder, DbError> {
        order.id = self.db_order_seq.next();
        println!("Inserting new order {:?}", order);
        self.db_orders.push(order);
        
        //TODO - just simple implementation
        Ok(self.db_orders.last().unwrap())
    }

    pub fn update(&mut self, id: i64, new_order: DbOrder) -> Result<(), DbError> {
        let index = self.db_orders.iter().position(|ord| ord.id == id);    

        match index {
            Some(ind) =>  { 
                self.db_orders[ind] = new_order;
                println!("Updated order {} at index: {} ",id, ind); },
            None => {
                println!("Order {} not found", id); 
                return Err(DbError::NotFound); }
        }
        Ok(())
    }

    pub fn get_orders<F>(&mut self, predicate: F) -> Vec<&mut DbOrder>
    where F: Fn(&DbOrder) -> bool{
        let mut result: Vec<&mut DbOrder> = Vec::new(); 
        for order in self.db_orders.iter_mut() {
            if predicate(&order) {
                result.push(order);
            }
        }
        result
    }



}


