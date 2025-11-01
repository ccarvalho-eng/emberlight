use rand::Rng;

pub struct Narrative;

impl Narrative {
    pub fn opening_verse() -> &'static str {
        "\n\x1b[36m╔═══════════════════════════════════════════════════════════╗\n\
         ║  In the twilight hours, where embers fade to darkness,   ║\n\
         ║  Through forgotten valleys and moonlit paths,            ║\n\
         ║  A warrior rises with steel and determination.           ║\n\
         ║  The cursed lands await liberation.                      ║\n\
         ╚═══════════════════════════════════════════════════════════╝\x1b[0m\n"
    }

    pub fn random_travel_verse() -> &'static str {
        let mut rng = rand::rng();
        let verses = [
            "\n\x1b[90mThe path winds through mist and ancient stone.\n\
             You travel alone with only your thoughts and blade.\x1b[0m\n",
            "\n\x1b[90mDeep in these woods, spirits watch from the shadows.\n\
             What forgotten truths lie buried here?\x1b[0m\n",
            "\n\x1b[90mThe wind carries whispers of those who fell before you.\n\
             Their courage failed them. Will yours endure?\x1b[0m\n",
            "\n\x1b[90mBeneath cold stars, the journey stretches onward.\n\
             Each step forward adds weight to your legend.\x1b[0m\n",
            "\n\x1b[90mThrough valleys carved by time and mountain peaks lost in clouds.\n\
             This is the path of heroes, and you walk it now.\x1b[0m\n",
        ];
        verses[rng.random_range(0..verses.len())]
    }

    pub fn victory_verse() -> &'static str {
        let mut rng = rand::rng();
        let verses = [
            "\n\x1b[33mSteel sings its song of finality.\n\
             The beast crumples, and silence returns to the world.\x1b[0m\n",
            "\n\x1b[33mDarkness retreats before your determination.\n\
             You stand victorious over the fallen.\x1b[0m\n",
            "\n\x1b[33mYour strike was swift and certain.\n\
             Another chapter added to the growing legend.\x1b[0m\n",
            "\n\x1b[33mWith each vanquished foe, your resolve strengthens.\n\
             Power flows to those who refuse to yield.\x1b[0m\n",
        ];
        verses[rng.random_range(0..verses.len())]
    }

    pub fn death_verse() -> &'static str {
        "\n\x1b[31m╔═══════════════════════════════════════════════════════════╗\n\
         ║  The ember fades. Your light grows dim.                  ║\n\
         ║  This chapter ends, but the story continues.             ║\n\
         ║  Heroes rise from ashes and defeat.                      ║\n\
         ║  Your legend is not yet written in full...               ║\n\
         ╚═══════════════════════════════════════════════════════════╝\x1b[0m\n"
    }

    pub fn boss_approach_verse() -> &'static str {
        "\n\x1b[35m⚠ The air grows heavy with ancient malevolence.\n\
         A great evil awaits just ahead.\n\
         Steel yourself. Destiny approaches.\n\
         What happens next will be remembered. ⚠\x1b[0m\n"
    }
}

#[derive(Debug, Clone)]
pub enum Location {
    DarkForest,
    AbandonedRuins,
    MountainPass,
    HauntedCrypt,
    VolcanicCavern,
    FrozenWastes,
    ShadowMarsh,
    DragonLair,
}

impl Location {
    pub fn random(encounter: u32) -> Self {
        let mut rng = rand::rng();

        if encounter > 0 && encounter % 5 == 0 {
            return Location::DragonLair;
        }

        let locations = [
            Location::DarkForest,
            Location::AbandonedRuins,
            Location::MountainPass,
            Location::HauntedCrypt,
            Location::VolcanicCavern,
            Location::FrozenWastes,
            Location::ShadowMarsh,
        ];

        locations[rng.random_range(0..locations.len())].clone()
    }

    pub fn description(&self) -> &str {
        match self {
            Location::DarkForest => {
                "\x1b[32m🌲 THE DARK FOREST 🌲\x1b[0m\n\
                 Ancient trees loom overhead, their gnarled branches blocking out the sun.\n\
                 Strange whispers echo through the undergrowth, and glowing eyes watch from the shadows."
            }
            Location::AbandonedRuins => {
                "\x1b[37m🏛️  ABANDONED RUINS 🏛️\x1b[0m\n\
                 Crumbling stone pillars mark what was once a grand temple.\n\
                 The air is heavy with forgotten magic, and strange symbols glow faintly on weathered walls."
            }
            Location::MountainPass => {
                "\x1b[90m⛰️  MOUNTAIN PASS ⛰️\x1b[0m\n\
                 A treacherous path winds between jagged peaks, where bitter winds howl.\n\
                 The bones of previous travelers lie scattered among the rocks."
            }
            Location::HauntedCrypt => {
                "\x1b[35m⚰️  HAUNTED CRYPT ⚰️\x1b[0m\n\
                 Stone tombs line the walls of this underground chamber.\n\
                 The stench of death fills your nostrils, and spectral moans drift through the air."
            }
            Location::VolcanicCavern => {
                "\x1b[31m🔥 VOLCANIC CAVERN 🔥\x1b[0m\n\
                 Rivers of molten lava illuminate the darkness with an infernal glow.\n\
                 The heat is oppressive, and the ground trembles beneath your feet."
            }
            Location::FrozenWastes => {
                "\x1b[36m❄️  FROZEN WASTES ❄️\x1b[0m\n\
                 An endless expanse of ice and snow stretches before you.\n\
                 The cold bites deep into your bones, and the howling wind threatens to freeze you solid."
            }
            Location::ShadowMarsh => {
                "\x1b[33m🌫️  SHADOW MARSH 🌫️\x1b[0m\n\
                 Thick fog obscures your vision as you wade through murky waters.\n\
                 Strange creatures lurk beneath the surface, and the smell of decay is overwhelming."
            }
            Location::DragonLair => {
                "\x1b[91m🐉 DRAGON'S LAIR 🐉\x1b[0m\n\
                 You stand before a massive cavern entrance, scorched earth all around.\n\
                 The air shimmers with heat, and piles of gold and bones glitter in the darkness.\n\
                 This is where legends are born... or die."
            }
        }
    }

    pub fn encounter_intro(&self) -> &str {
        match self {
            Location::DarkForest => "Something moves between the trees...",
            Location::AbandonedRuins => "A dark figure emerges from the shadows of the ruins...",
            Location::MountainPass => "A roar echoes from the cliffs above...",
            Location::HauntedCrypt => "The dead do not rest here...",
            Location::VolcanicCavern => "A beast born of flame approaches...",
            Location::FrozenWastes => "An icy horror materializes from the blizzard...",
            Location::ShadowMarsh => "The waters ripple as something rises...",
            Location::DragonLair => "The ground shakes. Wings unfold in the darkness. IT AWAKENS.",
        }
    }

    pub fn available_actions(&self) -> Vec<LocationAction> {
        match self {
            Location::DarkForest => vec![
                LocationAction::Search,
                LocationAction::Rest,
                LocationAction::Investigate,
            ],
            Location::AbandonedRuins => vec![
                LocationAction::Search,
                LocationAction::ReadRunes,
                LocationAction::Investigate,
            ],
            Location::MountainPass => vec![LocationAction::Search, LocationAction::Rest],
            Location::HauntedCrypt => vec![
                LocationAction::Search,
                LocationAction::Pray,
                LocationAction::Investigate,
            ],
            Location::VolcanicCavern => vec![LocationAction::Search, LocationAction::Investigate],
            Location::FrozenWastes => vec![LocationAction::Search, LocationAction::Rest],
            Location::ShadowMarsh => vec![LocationAction::Search, LocationAction::Investigate],
            Location::DragonLair => vec![LocationAction::Search, LocationAction::Prepare],
        }
    }
}

#[derive(Debug, Clone)]
pub enum LocationAction {
    Search,
    Rest,
    Investigate,
    ReadRunes,
    Pray,
    Prepare,
}

impl LocationAction {
    pub fn name(&self) -> &str {
        match self {
            LocationAction::Search => "Search for treasures",
            LocationAction::Rest => "Rest and gather strength",
            LocationAction::Investigate => "Investigate your surroundings",
            LocationAction::ReadRunes => "Attempt to read the ancient runes",
            LocationAction::Pray => "Pray to the forgotten gods",
            LocationAction::Prepare => "Prepare for the ultimate battle",
        }
    }

    pub fn execute(
        &self,
        player: &mut Box<dyn crate::character::traits::Fighter>,
        gold: &mut u32,
    ) -> String {
        let mut rng = rand::rng();

        match self {
            LocationAction::Search => {
                let luck = rng.random_range(1..=100);
                if luck > 70 {
                    let found_gold = rng.random_range(10..=30);
                    *gold += found_gold;
                    format!(
                        "\x1b[33m✨ You discover a hidden cache! Found {} gold coins!\x1b[0m",
                        found_gold
                    )
                } else if luck > 40 {
                    format!("\x1b[90mYou search carefully but find nothing of value.\x1b[0m")
                } else {
                    let damage = rng.random_range(5..=15);
                    player.take_damage(damage);
                    format!(
                        "\x1b[31m⚠ You trigger a trap! Lost {} health!\x1b[0m",
                        damage
                    )
                }
            }
            LocationAction::Rest => {
                let heal_amount = rng.random_range(15..=25);
                player.heal(heal_amount);
                format!(
                    "\x1b[32m😌 You rest briefly and recover {} health.\x1b[0m",
                    heal_amount
                )
            }
            LocationAction::Investigate => {
                let discovery = rng.random_range(1..=4);
                match discovery {
                    1 => format!("\x1b[36mYou notice strange markings on the ground. They seem to form a warning...\x1b[0m"),
                    2 => format!("\x1b[36mYou find traces of a previous battle. Whoever fought here did not survive.\x1b[0m"),
                    3 => {
                        player.heal(10);
                        format!("\x1b[32mYou discover a natural spring and drink deeply. Restored 10 health.\x1b[0m")
                    }
                    _ => format!("\x1b[36mYou learn the lay of the land, feeling more prepared for what's ahead.\x1b[0m"),
                }
            }
            LocationAction::ReadRunes => {
                let understanding = rng.random_range(1..=100);
                if understanding > 60 {
                    let resource_gain = 20;
                    format!(
                        "\x1b[35m✨ The ancient magic resonates with you! Restored {} {}!\x1b[0m",
                        resource_gain,
                        player.get_resource_name()
                    )
                } else {
                    format!(
                        "\x1b[90mThe runes are too ancient, their meaning lost to time...\x1b[0m"
                    )
                }
            }
            LocationAction::Pray => {
                let blessing = rng.random_range(1..=100);
                if blessing > 75 {
                    let heal_amount = rng.random_range(20..=40);
                    player.heal(heal_amount);
                    format!(
                        "\x1b[93m✨ The gods have heard your prayer! Restored {} health!\x1b[0m",
                        heal_amount
                    )
                } else if blessing > 30 {
                    format!("\x1b[90mYour prayers echo in the silence. Perhaps the gods are watching...\x1b[0m")
                } else {
                    format!("\x1b[90mOnly silence answers your prayers.\x1b[0m")
                }
            }
            LocationAction::Prepare => {
                player.heal(30);
                format!("\x1b[93m⚔️ You steel your nerves and prepare for the battle ahead. Restored 30 health.\x1b[0m")
            }
        }
    }
}

pub struct RandomEvent;

impl RandomEvent {
    pub fn try_trigger() -> Option<String> {
        let mut rng = rand::rng();

        if rng.random_range(1..=100) > 80 {
            let events = [
                "\x1b[36m🌟 A shooting star streaks across the sky. You feel strangely invigorated.\x1b[0m",
                "\x1b[90m🦅 A raven perches nearby, watching you with intelligent eyes before flying away.\x1b[0m",
                "\x1b[35m👻 You hear distant laughter, though no one is there...\x1b[0m",
                "\x1b[33m🍂 A cold wind blows, carrying whispers of forgotten names.\x1b[0m",
                "\x1b[32m🌿 You notice strange mushrooms glowing faintly in the darkness.\x1b[0m",
            ];
            Some(events[rng.random_range(0..events.len())].to_string())
        } else {
            None
        }
    }
}
