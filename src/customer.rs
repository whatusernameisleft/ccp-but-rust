use crate::tickets::{sellers::TicketSeller, Ticket};
use core::panic;
use std::fmt;

#[derive(Default)]
pub enum Status {
    #[default]
    Outside,
    Inside,
    Passenger,
}

#[derive(Default)]
pub struct Customer {
    pub id: i32,
    pub status: Status,
    pub ticket: Option<Ticket>,
}

// pub enum Customer {
//     Outside { id: i32 },
//     Inside { id: i32 },
//     Passenger { id: i32, ticket: Ticket },
// }

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.status {
                Status::Outside | Status::Inside => format!("Customer-{}", self.id),
                Status::Passenger => format!("Passenger-{}", self.id),
            }
        )
    }
}

impl Customer {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    pub fn enter_building(&mut self) {
        match self.status {
            Status::Outside => {
                println!("{self} has entered the building.");
                self.status = Status::Inside;
            }
            Status::Inside | Status::Passenger => {
                panic!("{self} is already inside so they can't enter the building again.")
            }
        }
    }

    pub fn buy_ticket(&mut self, ticket: Ticket, seller: &TicketSeller) {
        match self.status {
            Status::Inside => {
                println!("{self} has bought a {ticket} ticket from {seller}.");
                self.status = Status::Passenger;
                self.ticket = Some(ticket);
            }
            Status::Outside => {
                panic!("{self} is not in the building so they can't buy a ticket.")
            }
            Status::Passenger => {
                panic!("{self} already has a ticket so they can't buy another ticket.")
            }
        }
    }

    pub fn get_string(&self) -> String {
        match self.status {
            Status::Outside => format!("{self} is waiting outside of the building."),
            Status::Inside => format!("{self} is inside of the building."),
            Status::Passenger => format!(
                "{self} is has a ticket for {}.",
                self.ticket
                    .as_ref()
                    .unwrap_or_else(|| panic!("{self} has no ticket somehow."))
            ),
        }
    }
}
