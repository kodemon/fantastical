use cruet::Inflector;
use substring::Substring;

use crate::{
  resources::names::{Gender, Names, Size},
  utils::{is_vowel, roll_die, sample},
};

pub struct Species {
  names: Names,
}

impl Species {
  pub fn new() -> Self {
    Self {
      names: Names::new(),
    }
  }

  pub fn goblin(&self) -> String {
    return self.vile_and_crude(&Size::Small);
  }

  pub fn orc(&self) -> String {
    return self.vile_and_crude(&Size::Medium);
  }

  pub fn ogre(&self) -> String {
    return self.vile_and_crude(&Size::Large);
  }

  pub fn vile_and_crude(&self, size: &Size) -> String {
    let names = match size {
      Size::Small => &self.names.vile_and_crude.small,
      Size::Medium => &self.names.vile_and_crude.medium,
      Size::Large => &self.names.vile_and_crude.large,
    };

    let first = sample(&names);
    let second = sample(&names);

    return format!("{}{}", first, second).to_sentence_case();
  }

  pub fn cave_person(&self, gender: Gender) -> String {
    let resource = &self.names.primitive;

    let name = sample(&resource.names).to_sentence_case();
    let roll = roll_die(10);

    match gender {
      Gender::Male => {
        if roll > 3 || roll > 8 {
          return format!("{}-{}", name, sample(&resource.names).to_sentence_case());
        }
      }
      Gender::Female => {
        if roll > 5 {
          return format!("{}-{}", name, sample(&resource.names).to_sentence_case());
        }
        return format!("{}-{}", name, sample(&resource.suffixes).to_sentence_case());
      }
    }

    return format!("{}", name);
  }

  pub fn dwarf(&self, gender: Gender) -> String {
    let resource = &self.names.doughty;

    let name = sample(&resource.syllables).to_sentence_case();
    let roll = roll_die(10);

    match gender {
      Gender::Male => {
        if roll > 7 {
          let last_char = name.chars().last().unwrap();
          return format!("{}{}", name, if is_vowel(last_char) { 'r' } else { 'i' });
        }
        return format!("{}{}", name, sample(&resource.male_suffixes));
      }
      Gender::Female => {
        if roll > 7 {
          let last_char = name.chars().last().unwrap();
          return format!("{}{}", name, if is_vowel(last_char) { "ra" } else { "a" });
        }
        return format!("{}{}", name, sample(&resource.female_suffixes));
      }
    }
  }

  pub fn halfling(&self, gender: Gender) -> String {
    let name = format!(
      "{}{}",
      sample(&self.names.homely.syllables).to_sentence_case(),
      match gender {
        Gender::Male => sample(&self.names.homely.male_suffixes),
        Gender::Female => sample(&self.names.homely.female_suffixes),
      }
    );

    if roll_die(10) > 6 {
      return format!("{} {}", name, sample(&self.names.family_name.english));
    }

    return name;
  }

  pub fn gnome(&self, gender: Gender) -> String {
    let name = format!(
      "{}{}",
      sample(&self.names.doughty.syllables).to_sentence_case(),
      match gender {
        Gender::Male => sample(&self.names.homely.male_suffixes),
        Gender::Female => sample(&self.names.homely.female_suffixes),
      }
    );

    if roll_die(10) > 6 {
      return format!("{} {}", name, sample(&self.names.family_name.scottish));
    }

    return name;
  }

  pub fn elf(&self, gender: Gender) -> String {
    return format!(
      "{}{}{}",
      sample(&self.names.fair_and_noble.elf_prefixes).to_sentence_case(),
      sample(&self.names.fair_and_noble.middle),
      match gender {
        Gender::Male => sample(&self.names.fair_and_noble.male_suffixes),
        Gender::Female => sample(&self.names.fair_and_noble.female_suffixes),
      }
    );
  }

  pub fn high_elf(&self, gender: Gender) -> String {
    return format!(
      "{}{}{}",
      sample(&self.names.fair_and_noble.alternative_elf_prefixes).to_sentence_case(),
      sample(&self.names.fair_and_noble.middle),
      match gender {
        Gender::Male => sample(&self.names.fair_and_noble.male_suffixes),
        Gender::Female => sample(&self.names.fair_and_noble.female_suffixes),
      }
    );
  }

  pub fn fairy(&self, gender: Gender) -> String {
    return format!(
      "{}{}",
      sample(&self.names.fairy.prefixes).to_sentence_case(),
      match gender {
        Gender::Male => sample(&self.names.fairy.male_suffixes),
        Gender::Female => sample(&self.names.fairy.female_suffixes),
      }
    );
  }

  pub fn high_fairy(&self, gender: Gender) -> String {
    return format!(
      "{}{}",
      sample(&self.names.alternate_fairy.prefixes).to_sentence_case(),
      match gender {
        Gender::Male => sample(&self.names.alternate_fairy.male_suffixes),
        Gender::Female => sample(&self.names.alternate_fairy.female_suffixes),
      }
    );
  }

  pub fn dark_elf(&self, gender: Gender) -> String {
    let name = sample(&self.names.elegant_evil.prefixes_dark_elves).to_sentence_case();
    if roll_die(10) > 2 {
      return format!("{}{}", name, sample(&self.names.elegant_evil.middle));
    }
    return format!(
      "{}{}",
      name,
      match gender {
        Gender::Male => sample(&self.names.elegant_evil.male_suffixes),
        Gender::Female => sample(&self.names.elegant_evil.female_suffixes),
      }
    );
  }

  pub fn drow(&self, gender: Gender) -> String {
    let name = sample(&self.names.elegant_evil.prefixes_alternate_dark_elves).to_sentence_case();
    if roll_die(10) > 2 {
      return format!("{}{}", name, sample(&self.names.elegant_evil.middle));
    }
    return format!(
      "{}{}",
      name,
      match gender {
        Gender::Male => sample(&self.names.elegant_evil.male_suffixes),
        Gender::Female => sample(&self.names.elegant_evil.female_suffixes),
      }
    );
  }

  pub fn demon(&self) -> String {
    let chosen1 = InfernalSound::random();
    let chosen2 = InfernalSound::filtered_random(&chosen1);
    return format!(
      "{}{}",
      sample(match chosen1 {
        InfernalSound::Softs => &self.names.infernal.softs,
        InfernalSound::Dulls => &self.names.infernal.dulls,
        InfernalSound::Sharps => &self.names.infernal.sharps,
      })
      .to_title_case(),
      sample(match chosen2 {
        InfernalSound::Softs => &self.names.infernal.softs,
        InfernalSound::Dulls => &self.names.infernal.dulls,
        InfernalSound::Sharps => &self.names.infernal.sharps,
      })
    );
  }

  pub fn half_demon(&self, gender: Gender) -> String {
    return format!(
      "{}{}",
      sample(&self.names.malevolent.prefixes).to_sentence_case(),
      match gender {
        Gender::Male => sample(&self.names.malevolent.male_suffixes),
        Gender::Female => sample(&self.names.malevolent.female_suffixes),
      }
    );
  }

  pub fn dragon(&self, gender: Gender) -> String {
    let name = sample(&self.names.draconic.prefixes).to_sentence_case();
    let mut suffix = sample(&self.names.draconic.suffixes);

    if is_female(gender) {
      if suffix == "bazius" {
        suffix = String::from("bazia");
      } else if suffix.ends_with("os") {
        suffix = format!("{}{}", suffix, "sa");
      } else {
        suffix = String::from("is")
      }
    }

    return format!("{}{}", name, suffix);
  }

  pub fn angel(&self, gender: Gender) -> String {
    let mut name = sample(&self.names.empyreal.prefixes).to_sentence_case();

    if roll_die(12) == 1 {
      if is_female(gender) {
        let a_index = name.find("a").unwrap();
        name = format!(
          "{}{}{}",
          name.substring(0, a_index),
          "e",
          name.substring(a_index + 1, name.len())
        )
      }
      return format!(
        "{}{}",
        sample(&self.names.empyreal.titles).to_sentence_case(),
        name
      );
    }

    return format!(
      "{}{}",
      name,
      match gender {
        Gender::Male => sample(&self.names.empyreal.male_suffixes),
        Gender::Female => sample(&self.names.empyreal.female_suffixes),
      }
    );
  }

  pub fn human(&self, gender: Gender, multi: bool) -> String {
    if multi {
      return format!(
        "{} {}",
        match gender {
          Gender::Male => sample(&self.names.human.male),
          Gender::Female => sample(&self.names.human.female),
        },
        match roll_die(2) {
          1 => sample(&self.names.family_name.english),
          2 => sample(&self.names.family_name.scottish),
          _ => sample(&self.names.family_name.english),
        }
      );
    }
    match gender {
      Gender::Male => sample(&self.names.human.male),
      Gender::Female => sample(&self.names.human.female),
    }
  }
}

pub fn is_female(gender: Gender) -> bool {
  match gender {
    Gender::Female => true,
    Gender::Male => false,
  }
}

/*
|--------------------------------------------------------------------------------
| Infernal Sound
|--------------------------------------------------------------------------------
*/

enum InfernalSound {
  Softs,
  Dulls,
  Sharps,
}

impl InfernalSound {
  pub fn random() -> InfernalSound {
    match roll_die(3) {
      1 => InfernalSound::Softs,
      2 => InfernalSound::Dulls,
      3 => InfernalSound::Sharps,
      _ => InfernalSound::Softs,
    }
  }

  pub fn filtered_random(omit_sound: &InfernalSound) -> &InfernalSound {
    match omit_sound {
      InfernalSound::Softs => match roll_die(2) {
        1 => &InfernalSound::Dulls,
        2 => &InfernalSound::Sharps,
        _ => &InfernalSound::Dulls,
      },
      InfernalSound::Dulls => match roll_die(2) {
        1 => &InfernalSound::Softs,
        2 => &InfernalSound::Sharps,
        _ => &InfernalSound::Softs,
      },
      InfernalSound::Sharps => match roll_die(2) {
        1 => &InfernalSound::Softs,
        2 => &InfernalSound::Dulls,
        _ => &InfernalSound::Softs,
      },
    }
  }
}
