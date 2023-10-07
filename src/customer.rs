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

    pub fn enter_building(&mut self) -> Result<String, String> {
        match self.status {
            Status::Outside => {
                self.status = Status::Inside;
                Ok(format!("{self} has entered the building."))
            }
            Status::Inside | Status::Passenger => {
                Err(format!("{self} is already in the building."))
            }
        }
    }

    pub fn buy_ticket(&mut self, ticket: Ticket, seller: &TicketSeller) -> Option<String> {
        self.ticket = Some(ticket);
        Some(format!(
            "{self} has bought a {} ticket from {seller}.",
            self.ticket.as_ref()?
        ))
    }

    pub fn get_string(&self) -> String {
        match self.status {
            Status::Outside => format!("{self} is waiting outside of the building."),
            Status::Inside => format!("{self} is inside of the building."),
            Status::Passenger => match self.ticket {
                Some(ref ticket) => format!("{self} has a {ticket} ticket."),
                None => format!("Missing ticket error: {self} somehow doesn't have a ticket."),
            },
        }
    }
}
