use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::fmt;

pub mod sellers;

#[derive(Debug)]
pub enum Ticket {
    BukitBintang,
    KLTower,
    PetronasTwinTowers,
}

impl fmt::Display for Ticket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::BukitBintang => "Bukit Bintang",
                Self::KLTower => "KL Tower",
                Self::PetronasTwinTowers => "Petronas Twin Towers",
            }
        )
    }
}

impl Distribution<Ticket> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Ticket {
        match rng.gen_range(0..=2) {
            0 => Ticket::BukitBintang,
            1 => Ticket::KLTower,
            _ => Ticket::PetronasTwinTowers,
        }
    }
}
