use crate::species::Species;
use crate::agent::Agent;
use crate::card::Card;

pub struct Player {
  agent: Agent,
  pub species: Vec<Species>,
  pub hand: Vec<Card>
}

impl Player {
  pub fn new() -> Player {
    Player {agent: Agent {  }, species: vec![Species::new()], hand: vec![]}
  }
}