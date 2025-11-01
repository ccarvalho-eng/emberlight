use super::traits::{Character, Combat, Fighter, ResourcePool};

const BASE_HEALTH: u32 = 80;
const HEALTH_PER_LEVEL: u32 = 7;
const BASE_AGILITY: u32 = 18;
const BASE_STEALTH: u32 = 15;
const BASE_ENERGY: u32 = 100;
const ENERGY_PER_LEVEL: u32 = 10;
const BACKSTAB_COST: u32 = 40;
const BACKSTAB_CRIT_MULT: u32 = 3;

#[derive(Debug, Clone)]
pub struct Rogue {
    name: String,
    health: u32,
    max_health: u32,
    level: u32,
    agility: u32,
    stealth: u32,
    energy: u32,
    max_energy: u32,
}

impl Rogue {
    pub fn new(name: String, level: u32) -> Self {
        let max_health = BASE_HEALTH + (level * HEALTH_PER_LEVEL);
        let max_energy = BASE_ENERGY + (level * ENERGY_PER_LEVEL);
        Rogue {
            name,
            health: max_health,
            max_health,
            level,
            agility: BASE_AGILITY + level,
            stealth: BASE_STEALTH + level,
            energy: max_energy,
            max_energy,
        }
    }
}

impl Character for Rogue {
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

impl ResourcePool for Rogue {
    fn get_resource(&self) -> u32 {
        self.energy
    }

    fn get_max_resource(&self) -> u32 {
        self.max_energy
    }

    fn get_resource_name(&self) -> &str {
        "Energy"
    }

    fn set_resource(&mut self, amount: u32) {
        self.energy = amount.min(self.max_energy);
    }
}

impl Combat for Rogue {
    fn attack(&self) -> u32 {
        self.agility + self.stealth / 2
    }

    fn special_ability(&mut self) -> Option<u32> {
        if !self.consume_resource(BACKSTAB_COST) {
            println!("❌ {} doesn't have enough Energy!", self.name);
            return None;
        }

        let base_damage = self.agility + self.stealth / 2;
        let damage = base_damage * BACKSTAB_CRIT_MULT;
        println!(
            "🗡️  {} performs a Backstab - Critical Hit for {} damage! Energy: {}/{}",
            self.name, damage, self.energy, self.max_energy
        );
        Some(damage)
    }

    fn special_ability_cost(&self) -> u32 {
        BACKSTAB_COST
    }

    fn special_ability_name(&self) -> &str {
        "Backstab"
    }

    fn can_use_special(&self) -> bool {
        self.has_resource(BACKSTAB_COST)
    }
}

impl Fighter for Rogue {}
