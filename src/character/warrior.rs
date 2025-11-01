use super::traits::{Character, Combat, Fighter, ResourcePool};

const BASE_HEALTH: u32 = 100;
const HEALTH_PER_LEVEL: u32 = 10;
const BASE_STRENGTH: u32 = 15;
const BASE_ARMOR: u32 = 10;
const BASE_RAGE: u32 = 50;
const RAGE_PER_LEVEL: u32 = 5;
const SHIELD_BASH_COST: u32 = 20;
const SHIELD_BASH_DAMAGE_MULT: u32 = 2;

#[derive(Debug, Clone)]
pub struct Warrior {
    name: String,
    health: u32,
    max_health: u32,
    level: u32,
    strength: u32,
    #[allow(dead_code)]
    armor: u32,
    rage: u32,
    max_rage: u32,
}

impl Warrior {
    pub fn new(name: String, level: u32) -> Self {
        let max_health = BASE_HEALTH + (level * HEALTH_PER_LEVEL);
        let max_rage = BASE_RAGE + (level * RAGE_PER_LEVEL);
        Warrior {
            name,
            health: max_health,
            max_health,
            level,
            strength: BASE_STRENGTH + level,
            armor: BASE_ARMOR + level,
            rage: max_rage,
            max_rage,
        }
    }
}

impl Character for Warrior {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_health(&self) -> u32 {
        self.health
    }

    fn get_max_health(&self) -> u32 {
        self.max_health
    }

    fn get_level(&self) -> u32 {
        self.level
    }

    fn set_health(&mut self, health: u32) {
        self.health = health;
    }
}

impl ResourcePool for Warrior {
    fn get_resource(&self) -> u32 {
        self.rage
    }

    fn get_max_resource(&self) -> u32 {
        self.max_rage
    }

    fn get_resource_name(&self) -> &str {
        "Rage"
    }

    fn set_resource(&mut self, amount: u32) {
        self.rage = amount.min(self.max_rage);
    }
}

impl Combat for Warrior {
    fn attack(&self) -> u32 {
        self.strength + 10
    }

    fn special_ability(&mut self) -> Option<u32> {
        if !self.consume_resource(SHIELD_BASH_COST) {
            println!("❌ {} doesn't have enough Rage!", self.name);
            return None;
        }

        let damage = (self.strength + 10) * SHIELD_BASH_DAMAGE_MULT;
        println!(
            "⚔️  {} uses Shield Bash for {} damage! Rage: {}/{}",
            self.name, damage, self.rage, self.max_rage
        );
        Some(damage)
    }

    fn special_ability_cost(&self) -> u32 {
        SHIELD_BASH_COST
    }

    fn special_ability_name(&self) -> &str {
        "Shield Bash"
    }

    fn can_use_special(&self) -> bool {
        self.has_resource(SHIELD_BASH_COST)
    }
}

impl Fighter for Warrior {}
