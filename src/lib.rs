/*
|--------------------------------------------------------------------------------
| Modules
|--------------------------------------------------------------------------------
*/

pub mod generators;
pub mod inquire;
pub mod resources;
pub mod utils;

/*
|--------------------------------------------------------------------------------
| Exports
|--------------------------------------------------------------------------------
*/

use crate::generators::{parties::Parties, species::Species};

pub struct Fantastical {
  pub party: Parties,
  pub species: Species,
}

impl Fantastical {
  pub fn new() -> Fantastical {
    return Fantastical {
      party: Parties::new(),
      species: Species::new(),
    };
  }
}
