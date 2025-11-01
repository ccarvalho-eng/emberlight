// Base trait that all characters must implement
pub trait Character {
    fn get_name(&self) -> &str;
    fn get_health(&self) -> u32;
    fn get_max_health(&self) -> u32;
    fn get_level(&self) -> u32;

    fn take_damage(&mut self, damage: u32) {
        let current_health = self.get_health();
        let new_health = current_health.saturating_sub(damage);
        self.set_health(new_health);
        println!(
            "{} takes {} damage! HP: {}/{}",
            self.get_name(),
            damage,
            self.get_health(),
            self.get_max_health()
        );
    }

    fn heal(&mut self, amount: u32) {
        let current = self.get_health();
        let max = self.get_max_health();
        let new_health = (current + amount).min(max);
        self.set_health(new_health);
        println!(
            "{} heals for {}! HP: {}/{}",
            self.get_name(),
            amount,
            self.get_health(),
            self.get_max_health()
        );
    }

    fn set_health(&mut self, health: u32);

    fn is_alive(&self) -> bool {
        self.get_health() > 0
    }

    fn introduce(&self) {
        println!(
            "{} (Level {}) - HP: {}/{}",
            self.get_name(),
            self.get_level(),
            self.get_health(),
            self.get_max_health()
        );
    }
}

// Resource management for abilities
pub trait ResourcePool {
    fn get_resource(&self) -> u32;
    fn get_max_resource(&self) -> u32;
    fn get_resource_name(&self) -> &str;
    fn set_resource(&mut self, amount: u32);

    fn has_resource(&self, cost: u32) -> bool {
        self.get_resource() >= cost
    }

    fn consume_resource(&mut self, cost: u32) -> bool {
        if self.has_resource(cost) {
            self.set_resource(self.get_resource() - cost);
            true
        } else {
            false
        }
    }
}

// Combat trait for attack behavior
pub trait Combat {
    fn attack(&self) -> u32;
    fn special_ability(&mut self) -> Option<u32>; // Returns damage dealt, None if failed
    fn special_ability_cost(&self) -> u32;
    fn special_ability_name(&self) -> &str;
    fn can_use_special(&self) -> bool;
}

// Combined trait for characters that can fight (used for trait objects)
pub trait Fighter: Character + Combat + ResourcePool {}
