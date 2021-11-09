use rand::Rng;
use std::*;

pub struct Nim {
   nim_count: i8,
   user_first: bool,
   user_won: bool,
   game_done: bool,
}
impl Nim {
   pub fn new(nim_count: i8, user_first: bool) -> Self {
      Self {
         nim_count,
         user_first,
         user_won: false,
         game_done: false,
      }
   }

   pub fn play(&mut self) {
      println!("There are {} nims on the table!", self.nim_count);
      println!("----------------------------------------");

      if self.user_first {
         self.user_player_turn();
      }

      loop {
         self.other_player_turn();
         if self.game_done {
            break;
         }

         self.user_player_turn();
         if self.game_done {
            break;
         }
      }

      self.game_over();
   }

   fn user_player_turn(&mut self) {
      println!("Pick a number between 1 and 4");

      match get_input_i8() {
         Ok(c) => {
            if c < 1 || c > 4 {
               println!("Please enter a valid input!");
               self.user_player_turn();
               return;
            }

            self.nim_count -= c;

            println!("You took {} from the table!", c);
            println!("----------------------------------------");

            if self.nim_count <= 0 {
               self.user_won = true;
               self.game_done = true;
            }

            println!("New count: {}", self.nim_count);
         }
         Err(_) => {
            println!("Please enter a valid input!");
            self.user_player_turn();
            return;
         }
      }
   }

   fn other_player_turn(&mut self) {
      println!("Your opponent is making a move...");

      let one_sec = time::Duration::from_millis(1000);
      thread::sleep(one_sec);

      let take_num: i8;
      if self.nim_count < 5 {
         take_num = self.nim_count;
      } else {
         take_num = rand::thread_rng().gen_range(1..=4);
      }
      self.nim_count -= take_num;

      println!("Your opponent took {} from the table!", take_num);

      thread::sleep(one_sec);

      println!("----------------------------------------");

      if self.nim_count == 0 {
         self.game_done = true;
      }

      println!("New count: {}", self.nim_count);
   }

   fn game_over(&self) {
      if self.user_won {
         println!("Congratulations, you won!");
      } else {
         println!("Awww better luck next time!");
      }
   }
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
      Err(_) => return Err("Please enter a valid input!".to_string()),
   }
}
