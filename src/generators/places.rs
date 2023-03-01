use cruet::Inflector;

use crate::{resources::places::Tavern, utils::sample};

pub struct Places {
  pub tavern: Tavern,
}

impl Places {
  pub fn new() -> Places {
    return Places {
      tavern: Tavern::new(),
    };
  }

  pub fn tavern(&self) -> String {
    let mut pattern = sample(&self.tavern.patterns);

    for _ in 0..pattern.matches("<noun>").count() {
      pattern = pattern.replacen("<noun>", sample(&self.tavern.noun).as_str(), 1);
    }

    pattern = pattern.replace("<adjective>", sample(&self.tavern.adjective).as_str());
    pattern = pattern.replace("<title>", sample(&self.tavern.title).as_str());

    return pattern.to_title_case();
  }
}
