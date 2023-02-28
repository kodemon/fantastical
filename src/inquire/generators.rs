use inquire::Select;

use super::parties::select_party;
use super::species::select_species;

pub fn select_generator() {
  let answer = Select::new(
    "Which category do you want to generate for?",
    vec!["Parties", "Species"],
  )
  .prompt();

  match answer {
    Ok("Parties") => select_party(),
    Ok("Species") => select_species(),
    _ => println!("goto: None"),
  }
}
