/*
|--------------------------------------------------------------------------------
| Mystic Orders
|--------------------------------------------------------------------------------
*/

use crate::utils::random_bool;

pub struct MysticOrder {
  pub patterns: Vec<String>,
  pub group: MysticOrderGroup,
  pub description: MysticOrderDescription,
  pub entities: Vec<String>,
}

pub struct MysticOrderGroup {
  pub cliques: Vec<String>,
  pub people: Vec<String>,
}

pub struct MysticOrderDescription {
  pub quality: Vec<String>,
  pub color: Vec<String>,
}

impl MysticOrder {
  pub fn new() -> MysticOrder {
    MysticOrder {
      patterns: Vec::from([
        String::from("<group> of the <entity>"),
        String::from("<group> of the <description> <entity>"),
        String::from("<description> <group> of the <description> <entity>"),
        String::from("<description> <group>"),
      ]),
      group: MysticOrderGroup {
        cliques: Vec::from([
          String::from("alliance"),
          String::from("association"),
          String::from("band"),
          String::from("brotherhood"),
          String::from("cabal"),
          String::from("circle"),
          String::from("conclave"),
          String::from("confraternity"),
          String::from("convocation"),
          String::from("coterie"),
          String::from("fellowship"),
          String::from("fraternity"),
          String::from("guild"),
          String::from("league"),
          String::from("order"),
          String::from("siblingship"),
          String::from("sisterhood"),
          String::from("society"),
          String::from("sorority"),
        ]),
        people: Vec::from([
          String::from("adepts"),
          String::from("apostles"),
          String::from("aspirants"),
          String::from("brothers"),
          String::from("children"),
          String::from("colleagues"),
          String::from("devotees"),
          String::from("disciples"),
          String::from("fellows"),
          String::from("followers"),
          String::from("gentlemen"),
          String::from("illuminants"),
          String::from("initiates"),
          String::from("keepers"),
          String::from("ladies"),
          String::from("masters"),
          String::from("probers"),
          String::from("revealers"),
          String::from("seekers"),
          String::from("servants"),
          String::from("siblings"),
          String::from("sisters"),
          String::from("votaries"),
        ]),
      },
      description: MysticOrderDescription {
        quality: Vec::from([
          String::from("ancient"),
          String::from("arcane"),
          String::from("astral"),
          String::from("blinding"),
          String::from("bright"),
          String::from("brilliant"),
          String::from("burning"),
          String::from("bygone"),
          String::from("cardinal"),
          String::from("celestial"),
          String::from("cloudy"),
          String::from("concealed"),
          String::from("cosmic"),
          String::from("dark"),
          String::from("deep"),
          String::from("dexter"),
          String::from("difficult"),
          String::from("dusky"),
          String::from("effulgent"),
          String::from("elder"),
          String::from("elemental"),
          String::from("esoteric"),
          String::from("eternal"),
          String::from("ethereal"),
          String::from("existential"),
          String::from("forgotten"),
          String::from("gloomy"),
          String::from("glorious"),
          String::from("glowing"),
          String::from("gnostic"),
          String::from("hidden"),
          String::from("ineffable"),
          String::from("inner"),
          String::from("lost"),
          String::from("luminous"),
          String::from("lunar"),
          String::from("magical"),
          String::from("maieutical"),
          String::from("mysterious"),
          String::from("mystic"),
          String::from("occult"),
          String::from("penumbral"),
          String::from("profound"),
          String::from("pure"),
          String::from("quintessential"),
          String::from("radiant"),
          String::from("recondite"),
          String::from("resplendent"),
          String::from("revealed"),
          String::from("sacred"),
          String::from("secret"),
          String::from("shadowed"),
          String::from("shining"),
          String::from("sidereal"),
          String::from("singing"),
          String::from("sinister"),
          String::from("solemn"),
          String::from("spiral"),
          String::from("spiritual"),
          String::from("starry"),
          String::from("solar"),
          String::from("sublime"),
          String::from("supernal"),
          String::from("timeless"),
          String::from("transcendent"),
          String::from("true"),
          String::from("veiled"),
          String::from("zetetic"),
        ]),
        color: Vec::from([
          String::from("amber"),
          String::from("amethyst"),
          String::from("aquamarine"),
          String::from("azure"),
          String::from("beryl"),
          String::from("black"),
          String::from("blue"),
          String::from("brazen"),
          String::from("bronze"),
          String::from("brown"),
          String::from("carmine"),
          String::from("cerulean"),
          String::from("copper"),
          String::from("crimson"),
          String::from("crystal"),
          String::from("ebony"),
          String::from("emerald"),
          String::from("golden"),
          String::from("green"),
          String::from("grey"),
          String::from("incarnadine"),
          String::from("indigo"),
          String::from("ivory"),
          String::from("jade"),
          String::from("jet"),
          String::from("malachite"),
          String::from("orange"),
          String::from("pearly"),
          String::from("purple"),
          String::from("rainbow"),
          String::from("red"),
          String::from("rosy"),
          String::from("ruby"),
          String::from("russet"),
          String::from("sable"),
          String::from("sapphire"),
          String::from("scarlet"),
          String::from("silver"),
          String::from("topaz"),
          String::from("turquoise"),
          String::from("umber"),
          String::from("vermilion"),
          String::from("violaceous"),
          String::from("violet"),
          String::from("viridian"),
          String::from("white"),
          String::from("yellow"),
        ]),
      },
      entities: Vec::from([
        String::from("arcana"),
        String::from("beyond"),
        String::from("chalice"),
        String::from("chamber"),
        String::from("cloud"),
        String::from("cowl"),
        String::from("crown"),
        String::from("crystal"),
        String::from("darkness"),
        String::from("dawn"),
        String::from("day"),
        String::from("doctrine"),
        String::from("dominion"),
        String::from("energy"),
        String::from("enlightenment"),
        String::from("eye"),
        String::from("faith"),
        String::from("fane"),
        String::from("fire"),
        String::from("flame"),
        String::from("fountain"),
        String::from("gate"),
        String::from("glyph"),
        String::from("grail"),
        String::from("hand"),
        String::from("harmony"),
        String::from("heart"),
        String::from("helix"),
        String::from("influence"),
        String::from("insight"),
        String::from("key"),
        String::from("knowledge"),
        String::from("learning"),
        String::from("light"),
        String::from("lore"),
        String::from("mantle"),
        String::from("mastery"),
        String::from("mind"),
        String::from("moon"),
        String::from("mystery"),
        String::from("night"),
        String::from("orb"),
        String::from("path"),
        String::from("pentacle"),
        String::from("pillar"),
        String::from("pool"),
        String::from("portal"),
        String::from("power"),
        String::from("pyramid"),
        String::from("question"),
        String::from("radiance"),
        String::from("rainbow"),
        String::from("revelation"),
        String::from("robe"),
        String::from("rod"),
        String::from("sapience"),
        String::from("sceptre"),
        String::from("scroll"),
        String::from("secret"),
        String::from("shadow"),
        String::from("shrine"),
        String::from("sigil"),
        String::from("sign"),
        String::from("sky"),
        String::from("space"),
        String::from("sphere"),
        String::from("spring"),
        String::from("staff"),
        String::from("star"),
        String::from("stone"),
        String::from("sun"),
        String::from("symbol"),
        String::from("teaching"),
        String::from("temple"),
        String::from("throne"),
        String::from("time"),
        String::from("truth"),
        String::from("twilight"),
        String::from("veil"),
        String::from("verity"),
        String::from("void"),
        String::from("wand"),
        String::from("way"),
        String::from("wisdom"),
        String::from("word"),
        String::from("world"),
      ]),
    }
  }

  pub fn get_random_group(&self) -> &Vec<String> {
    if random_bool() {
      return &self.group.cliques;
    }
    return &self.group.people;
  }

  pub fn get_descriptions(&self) -> Vec<String> {
    return [
      self.description.quality.as_slice(),
      self.description.color.as_slice(),
    ]
    .concat();
  }
}

/*
|--------------------------------------------------------------------------------
| Military Unit
|--------------------------------------------------------------------------------
*/

pub struct MilitaryUnit {
  pub patterns: Vec<String>,
  pub groups: Vec<Vec<String>>,
  pub description: MilitaryUnitDescription,
  pub places: MilitaryUnitPlaces,
}

pub struct MilitaryUnitDescription {
  pub color: Vec<String>,
  pub other: Vec<String>,
}

pub struct MilitaryUnitPlaces {
  pub seas: Vec<String>,
  pub lands: Vec<String>,
}

impl MilitaryUnit {
  pub fn new() -> MilitaryUnit {
    MilitaryUnit {
      patterns: Vec::from([
        String::from("<commander>s <group>"),
        String::from("<color> <group>"),
        String::from("<other> <group>"),
        String::from("<color> <other> <group>"),
        String::from("<other> <color> <group>"),
        String::from("<group> of the <place>"),
      ]),
      groups: Vec::from([
        // team
        Vec::from([
          String::from("armada"),
          String::from("army"),
          String::from("battalion"),
          String::from("brigade"),
          String::from("cohort"),
          String::from("commandos"),
          String::from("company"),
          String::from("contingent"),
          String::from("division"),
          String::from("fleet"),
          String::from("force"),
          String::from("garrison"),
          String::from("guard"),
          String::from("legion"),
          String::from("militia"),
          String::from("patrol"),
          String::from("phalanx"),
          String::from("platoon"),
          String::from("regiment"),
          String::from("section"),
          String::from("sentinel"),
          String::from("sentry"),
          String::from("squad"),
          String::from("squadron"),
          String::from("troop"),
          String::from("vanguard"),
        ]),
        // soldiers
        Vec::from([
          String::from("avengers"),
          String::from("champions"),
          String::from("elite"),
          String::from("fighters"),
          String::from("janissaries"),
          String::from("marines"),
          String::from("paladins"),
          String::from("riders"),
          String::from("skirmishers"),
          String::from("soldiers"),
          String::from("troopers"),
          String::from("veterans"),
          String::from("victors"),
          String::from("warriors"),
        ]),
        // warders
        Vec::from([
          String::from("crusaders"),
          String::from("defenders"),
          String::from("guardians"),
          String::from("guards"),
          String::from("keepers"),
          String::from("knights"),
          String::from("lords"),
          String::from("preservers"),
          String::from("protectors"),
          String::from("rangers"),
          String::from("sentinels"),
          String::from("sentries"),
          String::from("wardens"),
          String::from("warders"),
          String::from("watchers"),
        ]),
        // mercenaries
        Vec::from([
          String::from("bandits"),
          String::from("destroyers"),
          String::from("devourers"),
          String::from("marauders"),
          String::from("pirates"),
          String::from("raptors"),
          String::from("reavers"),
        ]),
        // gear
        Vec::from([
          String::from("arrows"),
          String::from("axes"),
          String::from("blades"),
          String::from("bows"),
          String::from("bucklers"),
          String::from("claws"),
          String::from("daggers"),
          String::from("darts"),
          String::from("fangs"),
          String::from("fists"),
          String::from("flails"),
          String::from("gauntlets"),
          String::from("halberds"),
          String::from("hammers"),
          String::from("helms"),
          String::from("knives"),
          String::from("lances"),
          String::from("maces"),
          String::from("pikes"),
          String::from("scythes"),
          String::from("shields"),
          String::from("spears"),
          String::from("swords"),
          String::from("talons"),
          String::from("teeth"),
        ]),
        // creatures
        Vec::from([
          String::from("angels"),
          String::from("basilisks"),
          String::from("cobras"),
          String::from("demons"),
          String::from("devils"),
          String::from("eagles"),
          String::from("falcons"),
          String::from("griffins"),
          String::from("hawks"),
          String::from("hounds"),
          String::from("jaguars"),
          String::from("lions"),
          String::from("panthers"),
          String::from("rats"),
          String::from("scorpions"),
          String::from("sharks"),
          String::from("tigers"),
          String::from("vipers"),
          String::from("wolves"),
        ]),
      ]),
      description: MilitaryUnitDescription {
        color: Vec::from([
          String::from("black"),
          String::from("white"),
          String::from("red"),
          String::from("gold"),
          String::from("silver"),
          String::from("iron"),
          String::from("blue"),
          String::from("green"),
          String::from("grey"),
        ]),
        other: Vec::from([
          String::from("battle"),
          String::from("blood"),
          String::from("bolt"),
          String::from("bone"),
          String::from("chaos"),
          String::from("dark"),
          String::from("death"),
          String::from("dire"),
          String::from("doom"),
          String::from("fire"),
          String::from("flame"),
          String::from("free"),
          String::from("high"),
          String::from("law"),
          String::from("light"),
          String::from("lightning"),
          String::from("moon"),
          String::from("night"),
          String::from("rune"),
          String::from("sea"),
          String::from("skull"),
          String::from("star"),
          String::from("storm"),
          String::from("sun"),
          String::from("thunder"),
          String::from("thunderbolt"),
          String::from("torch"),
          String::from("war"),
          String::from("wave"),
          String::from("wind"),
          String::from("wing"),
          String::from("wrath"),
        ]),
      },
      places: MilitaryUnitPlaces {
        seas: Vec::from([
          String::from("billow"),
          String::from("breaker"),
          String::from("brine"),
          String::from("deep"),
          String::from("foam"),
          String::from("main"),
          String::from("ocean"),
          String::from("sea"),
          String::from("surf"),
          String::from("swell"),
          String::from("water"),
          String::from("wave"),
        ]),
        lands: Vec::from([
          String::from("cave"),
          String::from("cavern"),
          String::from("city"),
          String::from("crag"),
          String::from("dell"),
          String::from("desert"),
          String::from("earth"),
          String::from("forest"),
          String::from("grove"),
          String::from("hall"),
          String::from("hill"),
          String::from("hinterland"),
          String::from("isle"),
          String::from("lake"),
          String::from("land"),
          String::from("march"),
          String::from("marsh"),
          String::from("path"),
          String::from("plain"),
          String::from("province"),
          String::from("range"),
          String::from("river"),
          String::from("rock"),
          String::from("sand"),
          String::from("shore"),
          String::from("stone"),
          String::from("stream"),
          String::from("tower"),
          String::from("trail"),
          String::from("valley"),
          String::from("water"),
          String::from("way"),
          String::from("wood"),
        ]),
      },
    }
  }

  pub fn get_all_places(&self) -> Vec<String> {
    return [self.places.lands.as_slice(), self.places.seas.as_slice()].concat();
  }
}

/*
|--------------------------------------------------------------------------------
| Guilds
|--------------------------------------------------------------------------------
*/

pub struct Guilds {
  pub roles: Vec<String>,
  pub goals: Vec<String>,
  pub adjectives: Vec<String>,
  pub actions: Vec<String>,
  pub titles: Vec<String>,
  pub descriptions: Vec<String>,
  pub groups: Vec<Vec<String>>,
}

impl Guilds {
  pub fn new() -> Guilds {
    Guilds {
      roles: Vec::from([
        String::from("arrangers"),
        String::from("bestowers"),
        String::from("disbursers"),
        String::from("disposers"),
        String::from("harmonisers"),
        String::from("healers"),
        String::from("reconcilers"),
        String::from("regulators"),
        String::from("reinstaters"),
        String::from("restorers"),
      ]),
      goals: Vec::from([
        String::from("balance"),
        String::from("congruity"),
        String::from("correlation"),
        String::from("correspondence"),
        String::from("equilibrium"),
        String::from("equipoise"),
        String::from("equity"),
        String::from("equivalence"),
        String::from("parity"),
        String::from("symmetry"),
      ]),
      adjectives: Vec::from([
        String::from("acute"),
        String::from("apposite"),
        String::from("apt"),
        String::from("celestial"),
        String::from("cosmic"),
        String::from("decisive"),
        String::from("dependable"),
        String::from("eldritch"),
        String::from("elemental"),
        String::from("enchanted"),
        String::from("enigmatic"),
        String::from("ethereal"),
        String::from("extreme"),
        String::from("fabled"),
        String::from("faithful"),
        String::from("final"),
        String::from("fitting"),
        String::from("glittering"),
        String::from("impartial"),
        String::from("magisterial"),
        String::from("mythic"),
        String::from("mystic"),
        String::from("radiant"),
        String::from("reliable"),
        String::from("shadowy"),
        String::from("sorcerous"),
        String::from("supreme"),
        String::from("ultimate"),
        String::from("utmost"),
        String::from("wondrous"),
      ]),
      actions: Vec::from([
        String::from("action"),
        String::from("justice"),
        String::from("reckoning"),
        String::from("recompense"),
        String::from("redress"),
        String::from("reparation"),
        String::from("reprisal"),
        String::from("requital"),
        String::from("retribution"),
        String::from("satisfaction"),
        String::from("vindication"),
        String::from("blow"),
        String::from("execution"),
        String::from("expulsion"),
        String::from("forfeit"),
        String::from("judgment"),
        String::from("penalty"),
        String::from("punishment"),
        String::from("quittance"),
        String::from("revenge"),
        String::from("sentence"),
        String::from("settlement"),
        String::from("toll"),
        String::from("vengeance"),
      ]),
      titles: Vec::from([
        String::from("alliance"),
        String::from("association"),
        String::from("company"),
        String::from("corporation"),
        String::from("organisation"),
        String::from("society"),
        String::from("syndicate"),
        String::from("order"),
        String::from("brotherhood"),
        String::from("sisterhood"),
        String::from("conclave"),
        String::from("council"),
        String::from("league"),
        String::from("tribe"),
        String::from("clan"),
        String::from("assembly"),
        String::from("union"),
        String::from("faction"),
        String::from("confederation"),
      ]),
      descriptions: Vec::from([
        String::from("black"),
        String::from("cloud"),
        String::from("dark"),
        String::from("dim"),
        String::from("dusk"),
        String::from("fog"),
        String::from("gloom"),
        String::from("grey"),
        String::from("night"),
        String::from("shade"),
        String::from("shadow"),
        String::from("smoke"),
        String::from("quiet"),
        String::from("subtle"),
        String::from("whispering"),
        String::from("bloody"),
        String::from("hidden"),
        String::from("red"),
        String::from("ready"),
        String::from("sharp"),
        String::from("sudden"),
      ]),
      groups: Vec::from([
        // weapon
        Vec::from([
          String::from("arrow"),
          String::from("axe"),
          String::from("blade"),
          String::from("bow"),
          String::from("claw"),
          String::from("dagger"),
          String::from("dirk"),
          String::from("fang"),
          String::from("hand"),
          String::from("hammer"),
          String::from("javelin"),
          String::from("knife"),
          String::from("mace"),
          String::from("pike"),
          String::from("spear"),
          String::from("staff"),
          String::from("sword"),
          String::from("wand"),
        ]),
        // item
        Vec::from([
          String::from("amulet"),
          String::from("balance"),
          String::from("book"),
          String::from("cape"),
          String::from("cloak"),
          String::from("cowl"),
          String::from("hand"),
          String::from("hourglass"),
          String::from("hood"),
          String::from("key"),
          String::from("lantern"),
          String::from("mantle"),
          String::from("mask"),
          String::from("orb"),
          String::from("pendant"),
          String::from("quill"),
          String::from("ring"),
          String::from("scales"),
          String::from("scroll"),
          String::from("torch"),
        ]),
        // creature
        Vec::from([
          String::from("bat"),
          String::from("cat"),
          String::from("daw"),
          String::from("dog"),
          String::from("dragon"),
          String::from("owl"),
          String::from("pye"),
          String::from("rat"),
          String::from("scorpion"),
          String::from("snake"),
          String::from("spider"),
          String::from("weasel"),
          String::from("centaur"),
          String::from("dragon"),
          String::from("goblin"),
          String::from("griffin"),
          String::from("harpy"),
          String::from("mermaid"),
          String::from("minotaur"),
          String::from("pegasus"),
          String::from("unicorn"),
        ]),
        // action
        Vec::from([
          String::from("bring"),
          String::from("claim"),
          String::from("conquer"),
          String::from("destroy"),
          String::from("devour"),
          String::from("find"),
          String::from("hunt"),
          String::from("kill"),
          String::from("overthrow"),
          String::from("purge"),
          String::from("ravage"),
          String::from("reclaim"),
          String::from("search"),
          String::from("seek"),
          String::from("shadow"),
          String::from("slay"),
          String::from("stalk"),
          String::from("vanquish"),
          String::from("wipe"),
        ]),
      ]),
    }
  }
}