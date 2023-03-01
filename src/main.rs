use fantastical::Fantastical;

fn main() {
  // select_generator();

  let fantastical = Fantastical::new();

  print!("\nMystic Orders:\n\n");
  for _ in 0..10 {
    println!("  {}", fantastical.party.mystic_order());
  }

  print!("\nMilitary Units:\n\n");
  for _ in 0..10 {
    println!("  {}", fantastical.party.military_unit());
  }

  print!("\nGuilds:\n\n");
  for _ in 0..10 {
    println!("  {}", fantastical.party.guild());
  }

  print!("\nTaverns:\n\n");
  for _ in 0..10 {
    println!("  {}", fantastical.place.tavern());
  }

  print!("\nGoblins:\n\n");
  let mut goblins: Vec<String> = vec![];
  for _ in 0..5 {
    goblins.push(fantastical.species.goblin());
  }
  println!("  {}", goblins.join(", "));

  print!("\nOrc:\n\n");
  let mut orcs: Vec<String> = vec![];
  for _ in 0..5 {
    orcs.push(fantastical.species.orc());
  }
  println!("  {}", orcs.join(", "));

  print!("\nOgre:\n\n");
  let mut ogres: Vec<String> = vec![];
  for _ in 0..5 {
    ogres.push(fantastical.species.ogre());
  }
  println!("  {}", ogres.join(", "));

  print!("\n");

  // print!("\n\n");

  // println!(
  //   "Male Cave Person: {}",
  //   fantastical.species.cave_person(Gender::Male)
  // );
  // println!(
  //   "Female Cave Person: {}",
  //   fantastical.species.cave_person(Gender::Female)
  // );

  // print!("\n\n");

  // println!("Male Dwarf: {}", fantastical.species.dwarf(Gender::Male));
  // println!(
  //   "Female Dwarf: {}",
  //   fantastical.species.dwarf(Gender::Female)
  // );

  // print!("\n\n");

  // println!(
  //   "Male Halfling: {}",
  //   fantastical.species.halfling(Gender::Male)
  // );
  // println!(
  //   "Female Halfling: {}",
  //   fantastical.species.halfling(Gender::Female)
  // );

  // print!("\n\n");

  // println!("Male Gnome: {}", fantastical.species.gnome(Gender::Male));
  // println!(
  //   "Female Gnome: {}",
  //   fantastical.species.gnome(Gender::Female)
  // );

  // print!("\n\n");

  // println!("Male Elf: {}", fantastical.species.elf(Gender::Male));
  // println!("Female Elf: {}", fantastical.species.elf(Gender::Female));

  // print!("\n\n");

  // println!(
  //   "Male High Elf: {}",
  //   fantastical.species.high_elf(Gender::Male)
  // );
  // println!(
  //   "Female High Elf: {}",
  //   fantastical.species.high_elf(Gender::Female)
  // );

  // print!("\n\n");

  // println!("Male Fairy: {}", fantastical.species.fairy(Gender::Male));
  // println!(
  //   "Female Fairy: {}",
  //   fantastical.species.fairy(Gender::Female)
  // );

  // print!("\n\n");

  // println!(
  //   "Male High Fairy: {}",
  //   fantastical.species.high_fairy(Gender::Male)
  // );
  // println!(
  //   "Female High Fairy: {}",
  //   fantastical.species.high_fairy(Gender::Female)
  // );

  // print!("\n\n");

  // println!(
  //   "Male Dark Elf: {}",
  //   fantastical.species.dark_elf(Gender::Male)
  // );
  // println!(
  //   "Female Dark Elf: {}",
  //   fantastical.species.dark_elf(Gender::Female)
  // );

  // print!("\n\n");

  // println!("Male Drow: {}", fantastical.species.drow(Gender::Male));
  // println!("Female Drow: {}", fantastical.species.drow(Gender::Female));

  // print!("\n\n");

  // println!("Demon: {}", fantastical.species.demon());

  // print!("\n\n");

  // println!(
  //   "Male Half Demon: {}",
  //   fantastical.species.half_demon(Gender::Male)
  // );
  // println!(
  //   "Female Half Demon: {}",
  //   fantastical.species.half_demon(Gender::Female)
  // );

  // print!("\n\n");

  // println!("Male Dragon: {}", fantastical.species.dragon(Gender::Male));
  // println!(
  //   "Female Dragon: {}",
  //   fantastical.species.dragon(Gender::Female)
  // );

  // print!("\n\n");

  // println!("Male Angel: {}", fantastical.species.angel(Gender::Male));
  // println!(
  //   "Female Angel: {}",
  //   fantastical.species.angel(Gender::Female)
  // );

  // print!("\n\n");

  // println!(
  //   "Male Human: {}",
  //   fantastical.species.human(Gender::Male, random_bool())
  // );
  // println!(
  //   "Female Human: {}",
  //   fantastical.species.human(Gender::Female, random_bool())
  // );

  // print!("\n\n");
}
