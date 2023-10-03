use ccp_but_rust::customer;

#[tokio::main]
async fn main() {
    let customer = customer::Customer {
        id: 1,
        status: customer::CustomerStatus::Outside,
        ticket: Some(customer::Ticket::BukitBintang),
    };

    println!("{customer}");
}
