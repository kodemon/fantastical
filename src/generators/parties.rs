use crate::resources::names::Gender;
use crate::resources::parties::{Guilds, MilitaryUnit, MysticOrder};
use crate::utils::{random_bool, roll_die, sample};
use cruet::Inflector;

use super::species::Species;

pub struct Parties {
  pub military_unit: MilitaryUnit,
  pub mystic_order: MysticOrder,
  pub guilds: Guilds,
}

impl Parties {
  pub fn new() -> Parties {
    return Parties {
      military_unit: MilitaryUnit::new(),
      mystic_order: MysticOrder::new(),
      guilds: Guilds::new(),
    };
  }

  pub fn military_unit(&self) -> String {
    let commander = Species::new().human(Gender::random(), random_bool());
    let group = sample(&self.military_unit.groups);
    let color = &self.military_unit.description.color;
    let other = &self.military_unit.description.other;
    let place = &self.military_unit.get_all_places();

    // ### Apply Pattern

    let pattern = sample(&self.military_unit.patterns);
    let text = pattern
      .replace("<commander>", commander.as_str())
      .replace("<group>", sample(&group).as_str())
      .replace("<color>", sample(&color).as_str())
      .replace("<other>", sample(&other).as_str())
      .replace("<place>", sample(&place).as_str());

    // ### Return Capitalized Result

    return text.to_title_case();
  }

  pub fn mystic_order(&self) -> String {
    let descriptions = &self.mystic_order.get_descriptions();
    let mut pattern = sample(&self.mystic_order.patterns);

    for _ in 0..pattern.matches("<description>").count() {
      pattern = pattern.replacen("<description>", sample(&descriptions).as_str(), 1);
    }

    pattern = pattern.replace(
      "<group>",
      sample(&self.mystic_order.get_random_group()).as_str(),
    );
    pattern = pattern.replace("<entity>", sample(&self.mystic_order.entities).as_str());

    return pattern.to_title_case();
  }

  pub fn guild(&self) -> String {
    let roll = roll_die(10);
    if roll < 3 {
      return format!(
        "{} of {}",
        sample(&self.guilds.roles).to_sentence_case(),
        sample(&self.guilds.goals).to_sentence_case()
      );
    }
    if roll < 6 {
      return format!(
        "{} {} {}",
        sample(&self.guilds.adjectives).to_sentence_case(),
        sample(&self.guilds.actions).to_sentence_case(),
        sample(&self.guilds.titles).to_sentence_case(),
      );
    }
    return format!(
      "{} {}",
      sample(&self.guilds.descriptions).to_sentence_case(),
      sample(&sample(&self.guilds.groups)).to_sentence_case(),
    );
  }
}
