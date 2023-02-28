use std::process::exit;

use crate::Fantastical;
use inquire::Select;

pub fn select_party() {
  let answer = Select::new(
    "Which party type do you want to generate for?",
    vec!["Military Unit", "Mystic Order", "Guilds"],
  )
  .prompt();

  match answer {
    Ok("Military Unit") => {
      for _ in 0..10 {
        println!("    {}", Fantastical::new().party.military_unit());
      }
    }
    Ok("Mystic Order") => {
      for _ in 0..10 {
        println!("    {}", Fantastical::new().party.mystic_order());
      }
    }
    Ok("Guilds") => {
      for _ in 0..10 {
        println!("    {}", Fantastical::new().party.guild());
      }
    }
    _ => exit(0),
  }

  select_party();
}
