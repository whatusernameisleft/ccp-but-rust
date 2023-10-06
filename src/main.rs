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
    let c3 = Customer {
        id: 3,
        status: Status::Passenger,
        ticket: Some(Ticket::BukitBintang),
    };

    println!("{c1} is gaming");
    println!("{c2} is gaming 2");
    println!("{c3} is gaming 3");

    c1.enter_building();
    println!("{}", c1.get_string());
    let ticket_booth_1 = TicketSeller::new_booth("Ticket Booth 1");
    ticket_booth_1.sell_ticket(&mut c1);
    println!("{}", c1.get_string());
}
