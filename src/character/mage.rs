use super::traits::{Character, Combat, Fighter, ResourcePool};

const BASE_HEALTH: u32 = 60;
const HEALTH_PER_LEVEL: u32 = 5;
const BASE_MANA: u32 = 80;
const MANA_PER_LEVEL: u32 = 10;
const BASE_INTELLIGENCE: u32 = 20;
const FIREBALL_COST: u32 = 30;
const FIREBALL_DAMAGE_MULT: u32 = 3;

#[derive(Debug, Clone)]
pub struct Mage {
    name: String,
    health: u32,
    max_health: u32,
    level: u32,
    intelligence: u32,
    mana: u32,
    max_mana: u32,
}

impl Mage {
    pub fn new(name: String, level: u32) -> Self {
        let max_health = BASE_HEALTH + (level * HEALTH_PER_LEVEL);
        let max_mana = BASE_MANA + (level * MANA_PER_LEVEL);
        Mage {
            name,
            health: max_health,
            max_health,
            level,
            intelligence: BASE_INTELLIGENCE + level,
            mana: max_mana,
            max_mana,
        }
    }
}

impl Character for Mage {
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

impl ResourcePool for Mage {
    fn get_resource(&self) -> u32 {
        self.mana
    }

    fn get_max_resource(&self) -> u32 {
        self.max_mana
    }

    fn get_resource_name(&self) -> &str {
        "Mana"
    }

    fn set_resource(&mut self, amount: u32) {
        self.mana = amount.min(self.max_mana);
    }
}

impl Combat for Mage {
    fn attack(&self) -> u32 {
        self.intelligence * 2
    }

    fn special_ability(&mut self) -> Option<u32> {
        if !self.consume_resource(FIREBALL_COST) {
            println!("❌ {} doesn't have enough Mana!", self.name);
            return None;
        }

        let damage = self.intelligence * FIREBALL_DAMAGE_MULT;
        println!(
            "🔥 {} casts Fireball for {} damage! Mana: {}/{}",
            self.name, damage, self.mana, self.max_mana
        );
        Some(damage)
    }

    fn special_ability_cost(&self) -> u32 {
        FIREBALL_COST
    }

    fn special_ability_name(&self) -> &str {
        "Fireball"
    }

    fn can_use_special(&self) -> bool {
        self.has_resource(FIREBALL_COST)
    }
}

impl Fighter for Mage {}
