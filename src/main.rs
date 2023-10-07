use ccp_but_rust::customer::*;
use ccp_but_rust::tickets::sellers::TicketSeller;
use ccp_but_rust::tickets::*;

#[tokio::main]
async fn main() {
    let mut c1 = Customer::new(1);
    let c2 = Customer {
        id: 2,
        ..Default::default()
    };
    let mut c3 = Customer {
        id: 3,
        status: Status::Passenger,
        ticket: None,
    };

    println!("{c1} is gaming");
    println!("{c2} is gaming 2");
    println!("{c3} is gaming 3");

    match c1.enter_building() {
        Ok(res) => println!("{res}"),
        Err(err) => println!("Error entering building: {err}"),
    }
    println!("{}", c1.get_string());

    let ticket_booth_1 = TicketSeller::new_booth("Ticket Booth 1");
    match ticket_booth_1.sell_ticket(&mut c1) {
        Ok(res) => println!("{res}"),
        Err(err) => println!("Error selling ticket: {err}"),
    };
    println!("{}", c1.get_string());
}
