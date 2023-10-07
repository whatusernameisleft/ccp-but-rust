use crate::customer::{Customer, Status};
use std::fmt;

pub struct Booth {
    pub name: String,
}

pub struct Machine {
    pub name: String,
}

pub enum TicketSeller {
    Booth(Booth),
    Machine(Machine),
}

impl fmt::Display for TicketSeller {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Booth(booth) => booth.name.as_str(),
                Self::Machine(machine) => machine.name.as_str(),
            }
        )
    }
}

impl TicketSeller {
    pub fn new_booth(name: &str) -> Self {
        Self::Booth(Booth {
            name: String::from(name),
        })
    }

    pub fn new_machine(name: &'static str) -> Self {
        Self::Machine(Machine {
            name: String::from(name),
        })
    }

    pub fn sell_ticket(&self, customer: &mut Customer) -> Result<String, String> {
        match customer.status {
            Status::Outside => Err(format!("{customer} is not in the building.")),
            Status::Inside => {
                let ticket: super::Ticket = rand::random();
                let res = customer
                    .buy_ticket(ticket, self)
                    .ok_or(format!("{customer} failed to buy a ticket."))?;
                customer.status = Status::Passenger;
                Ok(res)
            }
            Status::Passenger => Err(format!("{customer} already has a ticket.")),
        }
    }
}
