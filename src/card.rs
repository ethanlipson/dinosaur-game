extern crate rand;

use rand::{Rand, Rng};

pub enum Card {
  Carnivore,
  DefensiveHerding,
  Cooperation
}

impl Rand for Card {
  fn rand<R: Rng>(rng: &mut R) -> Self {
    match rng.gen_range(0, 3) {
      0 => Card::Carnivore,
      1 => Card::DefensiveHerding,
      _ => Card::Cooperation
    }
  }
}