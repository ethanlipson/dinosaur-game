use crate::card::Card;

pub struct Species {
  pub population: i32,
  pub body_size: i32,
  pub current_food_level: i32,
  pub cards: Vec<Card>
}

impl Species {
  pub fn new() -> Species {
    Species {population: 1, body_size: 1, current_food_level: 0, cards: vec![]}
  }
}