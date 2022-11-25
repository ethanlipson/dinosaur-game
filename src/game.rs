use crate::species::Species;
use crate::player::Player;
use crate::card::Card;
use rand::{self, Rng};

pub struct Game {
  rng: rand::ThreadRng,
  players: Vec<Player>,
  first_player_index: i32
}

impl Game {
  fn new(num_players: i32) -> Game {
    let players = (0..num_players).map(|_| Player::new()).collect();
    Game {rng: rand::thread_rng(), players, first_player_index: 0}
  }

  fn deal_cards(self: &mut Self) {
    for player in self.players.iter_mut() {
      for _ in 0..(player.species.len() + 3) {
        let card: Card = self.rng.gen();
        player.hand.push(card);
      }
    }
  }
}