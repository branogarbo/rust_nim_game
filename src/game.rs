use rand::Rng;
use std::io;

pub struct Nim {
   obj_count: i8,
   player_first: bool,
   user_won: bool,
}
impl Nim {
   pub fn new(obj_count: i8, player_first: bool) -> Self {
      Self {
         obj_count,
         player_first,
         user_won: false,
      }
   }

   pub fn play(&mut self) {
      if self.player_first {
         self.user_player_turn();
      }

      while self.obj_count > 0 {
         self.other_player_turn();
         self.user_player_turn();
      }

      if self.user_won {
         println!("Congratulations, you won!");
      } else {
         println!("Awww better luck next time!");
      }
   }

   fn user_player_turn(&mut self) {
      println!("Pick a number between 1 and 4");

      match get_input_i8() {
         Ok(c) => {
            if c < 1 || c > 4 {
               panic!("⚠️  Please enter a valid input ⚠️"); // try to find better way, error handling
            }

            self.obj_count -= c;

            if self.obj_count <= 0 {
               self.user_won = true;
            }
         }
         Err(e) => {
            panic!("{}", e);
         }
      }
   }

   fn other_player_turn(&mut self) {
      // let ran_take_num = radn
   }

   fn game_over() {}
}

fn get_input_string() -> String {
   let mut input = String::new();

   io::stdin().read_line(&mut input).unwrap();

   return input.trim().to_string();
}

fn get_input_i8() -> Result<i8, String> {
   let i = get_input_string();

   match i.parse() {
      Ok(num) => return Ok(num),
      Err(_) => return Err("⚠️  Please enter a valid input ⚠️".to_string()),
   }
}
