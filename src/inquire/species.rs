use std::process::exit;

use crate::{resources::names::Size, Fantastical};
use inquire::Select;

pub fn select_species() {
  let answer = Select::new(
    "Which species do you want to generate for?",
    vec![
      "Goblin",
      "Orc",
      "Ogre",
      "Vile and Crude",
      "Cave Person",
      "Dwarf",
      "Halfling",
      "Gnome",
      "Elf",
      "High Elf",
      "Fairy",
      "High Fairy",
      "Dark Elf",
      "Drow",
      "Demon",
      "Half Demon",
      "Dragon",
      "Angel",
      "Human",
    ],
  )
  .prompt();

  match answer {
    Ok("Goblin") => {
      for _ in 0..10 {
        println!("    {}", Fantastical::new().species.goblin());
      }
    }
    Ok("Orc") => {
      for _ in 0..10 {
        println!("    {}", Fantastical::new().species.orc());
      }
    }
    Ok("Ogre") => {
      for _ in 0..10 {
        println!("    {}", Fantastical::new().species.ogre());
      }
    }
    Ok("Vile and Crude") => {
      let size = &get_size();
      for _ in 0..10 {
        println!("    {}", Fantastical::new().species.vile_and_crude(size));
      }
    }
    _ => exit(0),
  }

  select_species();
}

fn get_size() -> Size {
  match Select::new("Select species size", vec!["Small", "Medium", "Large"]).prompt() {
    Ok("Small") => Size::Small,
    Ok("Medium") => Size::Medium,
    Ok("Large") => Size::Large,
    _ => exit(0),
  }
}
