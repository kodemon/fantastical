use rand::{thread_rng, Rng};

pub fn sample<T>(vec: &Vec<T>) -> T
where
  T: Clone,
{
  vec[thread_rng().gen_range(0..vec.len())].clone()
}

pub fn roll_die(sides: u8) -> u8 {
  return thread_rng().gen_range(1..=sides);
}

pub fn is_vowel(c: char) -> bool {
  match c {
    'a' | 'e' | 'i' | 'o' | 'u' | 'y' => true,
    _ => false,
  }
}

pub fn random_bool() -> bool {
  return thread_rng().gen_bool(0.5);
}
