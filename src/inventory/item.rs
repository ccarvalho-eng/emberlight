#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    name: String,
    value: u32,
}

impl Item {
    pub fn new(name: String, value: u32) -> Self {
        Item { name, value }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> u32 {
        self.value
    }

    pub fn random_loot() -> Self {
        use rand::Rng;
        let mut rng = rand::rng();

        let loot_roll = rng.random_range(1..=100);

        if loot_roll > 95 {
            // Legendary item
            Item::new("Ancient Artifact".to_string(), 500)
        } else if loot_roll > 80 {
            // Rare items
            let items = [
                ("Elven Amulet", 200),
                ("Dragon Scale", 250),
                ("Enchanted Ring", 180),
                ("Mystic Tome", 220),
            ];
            let item = items[rng.random_range(0..items.len())];
            Item::new(item.0.to_string(), item.1)
        } else if loot_roll > 50 {
            // Common valuable items
            let items = [
                ("Silver Dagger", 80),
                ("Gold Coin Pouch", 100),
                ("Gem", 90),
                ("Healing Herb", 60),
            ];
            let item = items[rng.random_range(0..items.len())];
            Item::new(item.0.to_string(), item.1)
        } else if loot_roll > 25 {
            // Health potion
            Item::new("Health Potion".to_string(), 50)
        } else {
            // Resource potion
            Item::new("Energy Potion".to_string(), 40)
        }
    }

    pub fn is_health_potion(&self) -> bool {
        self.name == "Health Potion"
    }

    pub fn is_energy_potion(&self) -> bool {
        self.name == "Energy Potion"
    }
}
