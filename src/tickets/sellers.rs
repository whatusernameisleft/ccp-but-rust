use crate::customer::Customer;
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

    pub fn new_machine(name: &str) -> Self {
        Self::Machine(Machine {
            name: String::from(name),
        })
    }

    pub fn sell_ticket(&self, customer: &mut Customer) {
        let ticket: super::Ticket = rand::random();
        customer.buy_ticket(ticket, self);
    }
}
