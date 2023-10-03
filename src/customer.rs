use std::fmt;

#[derive(Debug)]
pub enum CustomerStatus {
    Outside,
    Customer,
    Passenger,
}

#[derive(Debug)]
pub enum Ticket {
    BukitBintang,
    KLTower,
    PetronasTwinTowers,
}

#[derive(Debug)]
pub struct Customer {
    pub id: i32,
    pub status: CustomerStatus,
    pub ticket: Option<Ticket>,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.ticket {
            Some(ticket) => write!(
                f,
                "Customer-{} has status {:?} and has a {:?} ticket.",
                self.id, self.status, ticket,
            ),
            None => write!(
                f,
                "Customer-{} has status {:?} and does not have a ticket.",
                self.id, self.status,
            ),
        }
    }
}
