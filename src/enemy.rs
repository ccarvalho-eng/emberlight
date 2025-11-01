use crate::character::traits::{Character, Combat, Fighter, ResourcePool};

const BASE_HEALTH: u32 = 50;
const HEALTH_PER_LEVEL: u32 = 15;
const BASE_ATTACK: u32 = 10;
const ATTACK_PER_LEVEL: u32 = 3;
// Enemies don't use resources, so they have infinite "rage" conceptually
const ENEMY_RAGE: u32 = 999;

#[derive(Debug, Clone)]
pub struct Enemy {
    name: String,
    health: u32,
    max_health: u32,
    level: u32,
    attack_power: u32,
}

impl Enemy {
    pub fn new(name: String, level: u32) -> Self {
        let max_health = BASE_HEALTH + (level * HEALTH_PER_LEVEL);
        Enemy {
            name,
            health: max_health,
            max_health,
            level,
            attack_power: BASE_ATTACK + (level * ATTACK_PER_LEVEL),
        }
    }

    pub fn goblin(level: u32) -> Self {
        Self::new(String::from("Goblin"), level)
    }

    pub fn orc(level: u32) -> Self {
        let mut enemy = Self::new(String::from("Orc"), level);
        enemy.max_health += 20;
        enemy.health = enemy.max_health;
        enemy.attack_power += 5;
        enemy
    }

    pub fn dragon(level: u32) -> Self {
        let mut enemy = Self::new(String::from("Dragon"), level);
        enemy.max_health += 50;
        enemy.health = enemy.max_health;
        enemy.attack_power += 15;
        enemy
    }
}

impl Character for Enemy {
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

impl ResourcePool for Enemy {
    fn get_resource(&self) -> u32 {
        ENEMY_RAGE
    }

    fn get_max_resource(&self) -> u32 {
        ENEMY_RAGE
    }

    fn get_resource_name(&self) -> &str {
        "Rage"
    }

    fn set_resource(&mut self, _amount: u32) {
        // Enemies don't manage resources
    }
}

impl Combat for Enemy {
    fn attack(&self) -> u32 {
        self.attack_power
    }

    fn special_ability(&mut self) -> Option<u32> {
        // Enemies don't use special abilities in combat
        None
    }

    fn special_ability_cost(&self) -> u32 {
        0
    }

    fn special_ability_name(&self) -> &str {
        "Roar"
    }

    fn can_use_special(&self) -> bool {
        false
    }
}

impl Fighter for Enemy {}
