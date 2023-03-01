/*
|--------------------------------------------------------------------------------
| Modules
|--------------------------------------------------------------------------------
*/

pub mod generators;
pub mod resources;
pub mod utils;

/*
|--------------------------------------------------------------------------------
| Exports
|--------------------------------------------------------------------------------
*/

use crate::generators::{parties::Parties, places::Places, species::Species};

pub struct Fantastical {
  pub party: Parties,
  pub species: Species,
  pub place: Places,
}

impl Fantastical {
  pub fn new() -> Fantastical {
    return Fantastical {
      party: Parties::new(),
      species: Species::new(),
      place: Places::new(),
    };
  }
}
