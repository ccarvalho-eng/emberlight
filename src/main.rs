mod character;
mod enemy;
mod inventory;
mod narrative;
mod ui;

use std::io::{self, Write};

use rand::Rng;

use character::{Character, Combat, Fighter, Mage, Rogue, Warrior};
use enemy::Enemy;
use inventory::Item;
use narrative::{Location, Narrative, RandomEvent};
use ui::{
    print_banner, print_divider, print_enemy_stats, print_health_bar, print_resource_bar,
    show_character_stats,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    run_game()
}

fn run_game() -> Result<()> {
    print_banner();
    println!("{}", Narrative::opening_verse());

    // Character selection
    let mut player_character = select_character();
    player_character.introduce();

    // Give starting items
    println!("\nYou receive a Health Potion and an Energy Potion!");

    const STARTING_GOLD: u32 = 50;
    const POTION_VALUE: u32 = 50;
    const ENERGY_POTION_VALUE: u32 = 40;

    let mut gold = STARTING_GOLD;
    let mut inventory = vec![
        Item::new(String::from("Health Potion"), POTION_VALUE),
        Item::new(String::from("Energy Potion"), ENERGY_POTION_VALUE),
    ];

    println!("\n=== Your Adventure Begins! ===\n");

    // Battle loop
    let mut battle_count = 0;
    loop {
        battle_count += 1;

        // Random travel event
        if let Some(event) = RandomEvent::try_trigger() {
            println!("{}", event);
            std::thread::sleep(std::time::Duration::from_millis(800));
        }

        // Travel verse
        println!("{}", Narrative::random_travel_verse());
        std::thread::sleep(std::time::Duration::from_millis(600));

        print_divider();
        println!("╔════════════════════════════════════════╗");
        println!("║       ENCOUNTER #{:<2}                   ║", battle_count);
        println!("╚════════════════════════════════════════╝\n");

        // Location-based encounter
        let location = Location::random(battle_count);
        println!("{}\n", location.description());

        // Show location actions
        let actions = location.available_actions();
        if !actions.is_empty() {
            println!("┌────────────────────────────────────────┐");
            println!("│  What would you like to do?            │");
            println!("├────────────────────────────────────────┤");
            for (i, action) in actions.iter().enumerate() {
                println!("│  [{}] {:<35}│", i + 1, action.name());
            }
            println!("│  [0] Press onward                      │");
            println!("└────────────────────────────────────────┘");
            print!("\n➤ Choose: ");
            let _ = io::stdout().flush();

            let max_choice = actions.len() as u32;
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                if let Ok(choice) = input.trim().parse::<u32>() {
                    if choice > 0 && choice <= max_choice {
                        let action = &actions[(choice - 1) as usize];
                        let result = action.execute(&mut player_character, &mut gold);
                        println!("\n{}\n", result);
                        std::thread::sleep(std::time::Duration::from_millis(800));
                    }
                }
            }
        }

        // Boss warning
        if battle_count % 5 == 0 {
            println!("{}", Narrative::boss_approach_verse());
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }

        println!("\n{}", location.encounter_intro());
        let mut enemy = spawn_enemy(battle_count);
        println!(
            "\n⚠️  A wild {} (Level {}) appears!",
            enemy.get_name(),
            enemy.get_level()
        );
        print_enemy_stats(&enemy);

        // Battle
        let won = battle(&mut player_character, &mut enemy, &mut inventory);

        if !won {
            println!("{}", Narrative::death_verse());
            print_divider();
            println!("╔════════════════════════════════════════╗");
            println!("║          GAME OVER                     ║");
            println!("╠════════════════════════════════════════╣");
            println!(
                "║  You survived {} encounter(s)          ║",
                battle_count - 1
            );
            println!("╚════════════════════════════════════════╝");
            break;
        }

        // Victory rewards
        println!("{}", Narrative::victory_verse());
        let reward_gold = 20 + (battle_count * 5);
        gold += reward_gold;
        println!("\n╔════════════════════════════════════════╗");
        println!("║            VICTORY!                    ║");
        println!("╠════════════════════════════════════════╣");
        println!("║  Reward: +{} gold                     ║", reward_gold);
        println!("║  Total Gold: {} coins                 ║", gold);
        println!("╚════════════════════════════════════════╝");

        // Random loot with better chance after boss battles
        let mut rng = rand::rng();
        let loot_chance = if battle_count % 5 == 0 {
            100 // Always drop loot after boss
        } else {
            rng.random_range(1..=100)
        };

        if loot_chance > 40 {
            let item = Item::random_loot();
            let rarity = if item.value() >= 500 {
                "\x1b[95m★★★ LEGENDARY ★★★\x1b[0m"
            } else if item.value() >= 150 {
                "\x1b[93m★★ RARE ★★\x1b[0m"
            } else if item.value() >= 60 {
                "\x1b[92m★ UNCOMMON ★\x1b[0m"
            } else {
                "Common"
            };

            println!(
                "\n🎁 {} You found a \x1b[96m{}\x1b[0m! (Value: {}g)",
                rarity,
                item.name(),
                item.value()
            );
            inventory.push(item);
        } else {
            println!("\n💨 The enemy had no loot to speak of.");
        }

        // Main menu
        loop {
            print_divider();
            println!("┌────────────────────────────────────────┐");
            println!("│  MAIN MENU                             │");
            println!("├────────────────────────────────────────┤");
            println!("│  [1] ⚔️  Continue Quest                │");
            println!("│  [2] 📊 Stats                          │");
            println!("│  [3] 🎒 Inventory                      │");
            println!("│  [4] 🏠 Rest & Quit                    │");
            println!("└────────────────────────────────────────┘");
            print!("\n➤ Choose (1-4): ");
            let _ = io::stdout().flush();

            match get_user_choice(4) {
                1 => break, // Continue to next encounter
                2 => {
                    show_character_stats(&*player_character, gold, battle_count);
                }
                3 => {
                    use_inventory_menu(&mut player_character, &mut inventory, gold);
                }
                4 => {
                    print_divider();
                    println!("╔════════════════════════════════════════╗");
                    println!("║        ADVENTURE COMPLETE!             ║");
                    println!("╠════════════════════════════════════════╣");
                    show_character_stats(&*player_character, gold, battle_count);
                    println!(
                        "║  Items Collected: {:<2}                 ║",
                        inventory.len()
                    );
                    println!("╚════════════════════════════════════════╝");
                    println!("\n🏰 You rest at the tavern. Safe travels, hero!\n");
                    return Ok(());
                }
                _ => unreachable!(),
            }
        }
    }
    Ok(())
}

fn select_character() -> Box<dyn Fighter> {
    println!("┌────────────────────────────────────────┐");
    println!("│  SELECT YOUR CLASS                     │");
    println!("├────────────────────────────────────────┤");
    println!("│  [1] ⚔️  WARRIOR - Tank & Heavy Hitter │");
    println!("│  [2] 🔮 MAGE    - Spell Caster         │");
    println!("│  [3] 🗡️  ROGUE   - Swift Assassin      │");
    println!("└────────────────────────────────────────┘");
    print!("\n➤ Choose (1-3): ");
    let _ = io::stdout().flush();

    match get_user_choice(3) {
        1 => {
            let name = get_player_name();
            Box::new(Warrior::new(name, 1))
        }
        2 => {
            let name = get_player_name();
            Box::new(Mage::new(name, 1))
        }
        _ => {
            let name = get_player_name();
            Box::new(Rogue::new(name, 1))
        }
    }
}

fn get_player_name() -> String {
    print!("Enter your character's name: ");
    let _ = io::stdout().flush();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap_or_else(|_| {
        eprintln!("Failed to read input, using default name");
        0
    });
    let name = name.trim();
    if name.is_empty() {
        "Hero".to_string()
    } else {
        name.to_string()
    }
}

fn get_user_choice(max: u32) -> u32 {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        match input.trim().parse::<u32>() {
            Ok(num) if num >= 1 && num <= max => return num,
            _ => {
                print!("Invalid choice. Please enter 1-{}: ", max);
                let _ = io::stdout().flush();
            }
        }
    }
}

fn spawn_enemy(encounter: u32) -> Enemy {
    let mut rng = rand::rng();

    // Boss every 5 encounters
    if encounter % 5 == 0 {
        let level = encounter / 5 + 2;
        return Enemy::dragon(level);
    }

    // Random enemy with level scaling
    let level = 1 + (encounter / 2);
    let enemy_type = rng.random_range(1..=100);

    if enemy_type <= 50 {
        Enemy::goblin(level)
    } else if enemy_type <= 85 {
        Enemy::orc(level)
    } else {
        Enemy::dragon(level)
    }
}

fn battle(player: &mut Box<dyn Fighter>, enemy: &mut Enemy, inventory: &mut Vec<Item>) -> bool {
    println!("\n╔════════════════════════════════════════╗");
    println!("║        ⚔️  BATTLE START! ⚔️            ║");
    println!("╚════════════════════════════════════════╝\n");

    loop {
        // Player turn
        let ability_cost = player.special_ability_cost();
        let can_use = player.can_use_special();
        let ability_name = player.special_ability_name();
        let resource_name = player.get_resource_name();

        println!("\n┌────────────────────────────────────────┐");
        println!("│  YOUR TURN                             │");
        println!("├────────────────────────────────────────┤");
        print_health_bar("You", player.get_health(), player.get_max_health());
        print_resource_bar(
            resource_name,
            player.get_resource(),
            player.get_max_resource(),
        );
        print_health_bar(enemy.get_name(), enemy.get_health(), enemy.get_max_health());
        println!("├────────────────────────────────────────┤");
        println!("│  [1] ⚔️  Attack                        │");

        if can_use {
            println!(
                "│  [2] ✨ {} (-{} {})              │",
                ability_name, ability_cost, resource_name
            );
        } else {
            println!(
                "│  [2] ✨ {} (Need {} {})          │",
                ability_name, ability_cost, resource_name
            );
        }

        println!("│  [3] 🧪 Potion                         │");
        println!("│  [4] 🎒 Inventory                      │");
        println!("└────────────────────────────────────────┘");
        print!("\n➤ Action (1-4): ");
        let _ = io::stdout().flush();

        match get_user_choice(4) {
            1 => {
                let mut rng = rand::rng();
                let base_damage = player.attack();
                let variance = rng.random_range(0.85..=1.15);
                let mut damage = (base_damage as f32 * variance) as u32;

                // Critical hit chance
                let crit_chance = rng.random_range(1..=100);
                if crit_chance > 85 {
                    damage = (damage as f32 * 1.5) as u32;
                    println!("\n\x1b[91m⚡ CRITICAL HIT! ⚡\x1b[0m");
                    println!("💥 You attack for \x1b[93m{} damage!\x1b[0m", damage);
                } else {
                    println!("\n💥 You attack for {} damage!", damage);
                }
                enemy.take_damage(damage);
            }
            2 => {
                if let Some(base_damage) = player.special_ability() {
                    let mut rng = rand::rng();
                    let variance = rng.random_range(0.9..=1.1);
                    let damage = (base_damage as f32 * variance) as u32;
                    enemy.take_damage(damage);
                } else {
                    println!("\n⚠️  Turn wasted!");
                }
            }
            3 => {
                if use_health_potion(player, inventory) {
                    println!("\n✨ You used a Health Potion!");
                } else {
                    println!("\n❌ No Health Potions available! Turn wasted.");
                }
            }
            4 => {
                if show_and_use_inventory(player, inventory) {
                    // Item was used, end turn
                } else {
                    // No item used, don't end turn
                    continue;
                }
            }
            _ => unreachable!(),
        }

        // Check if enemy is defeated
        if !enemy.is_alive() {
            println!("\n🎉 {} has been defeated!", enemy.get_name());
            return true;
        }

        // Enemy turn
        println!("\n╔════════════════════════════════════════╗");
        println!("║          ENEMY TURN                    ║");
        println!("╚════════════════════════════════════════╝");

        let mut rng = rand::rng();
        let base_damage = enemy.attack();
        let variance = rng.random_range(0.8..=1.2);
        let mut damage = (base_damage as f32 * variance) as u32;

        // Enemy critical hit chance (lower than player)
        let crit_chance = rng.random_range(1..=100);
        if crit_chance > 92 {
            damage = (damage as f32 * 1.5) as u32;
            println!(
                "\n\x1b[91m⚡ {} lands a CRITICAL HIT! ⚡\x1b[0m",
                enemy.get_name()
            );
            println!("💥 You take \x1b[91m{} damage!\x1b[0m", damage);
        } else {
            println!("💥 {} attacks for {} damage!", enemy.get_name(), damage);
        }
        player.take_damage(damage);

        // Check if player is defeated
        if !player.is_alive() {
            return false;
        }
    }
}

fn use_health_potion(player: &mut Box<dyn Fighter>, inventory: &mut Vec<Item>) -> bool {
    const POTION_HEAL: u32 = 30;
    if let Some(pos) = inventory
        .iter()
        .position(|item| item.name() == "Health Potion")
    {
        inventory.remove(pos);
        player.heal(POTION_HEAL);
        true
    } else {
        false
    }
}

fn use_inventory_menu(player: &mut Box<dyn Fighter>, inventory: &mut Vec<Item>, gold: u32) {
    loop {
        println!("\n┌────────────────────────────────────────┐");
        println!("│  🎒 INVENTORY                          │");
        println!("├────────────────────────────────────────┤");
        println!("│  💰 Gold: {:<30}│", format!("{} coins", gold));
        println!(
            "│  ❤️  Health: {:<28}│",
            format!("{}/{}", player.get_health(), player.get_max_health())
        );
        println!(
            "│  ⚡ {}: {:<24}│",
            player.get_resource_name(),
            format!("{}/{}", player.get_resource(), player.get_max_resource())
        );
        println!("├────────────────────────────────────────┤");

        if inventory.is_empty() {
            println!("│  (Empty)                               │");
            println!("├────────────────────────────────────────┤");
            println!("│  [0] Back to Menu                      │");
            println!("└────────────────────────────────────────┘");
            print!("\n➤ Press 0 to go back: ");
            let _ = io::stdout().flush();
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
            return;
        }

        for (i, item) in inventory.iter().enumerate() {
            let usable = if item.is_health_potion() {
                if player.get_health() < player.get_max_health() {
                    "✓ Use"
                } else {
                    "✗ Full HP"
                }
            } else if item.is_energy_potion() {
                if player.get_resource() < player.get_max_resource() {
                    "✓ Use"
                } else {
                    "✗ Full"
                }
            } else {
                "View"
            };
            println!("│  [{}] {:<25} {:>7} │", i + 1, item.name(), usable);
        }

        println!("├────────────────────────────────────────┤");
        println!("│  [0] Back to Menu                      │");
        println!("└────────────────────────────────────────┘");
        print!("\n➤ Choose item to use (or 0 to go back): ");
        let _ = io::stdout().flush();

        let max_choice = inventory.len() as u32;
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        match input.trim().parse::<u32>() {
            Ok(0) => return, // Back to menu
            Ok(num) if num >= 1 && num <= max_choice => {
                let index = (num - 1) as usize;
                let item = &inventory[index];

                if item.is_health_potion() {
                    if player.get_health() >= player.get_max_health() {
                        println!("\n⚠️  You're already at full health!");
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        continue;
                    }
                    const POTION_HEAL: u32 = 30;
                    inventory.remove(index);
                    player.heal(POTION_HEAL);
                    println!(
                        "\n✨ You drink the Health Potion and restore {} HP!",
                        POTION_HEAL
                    );
                    println!(
                        "   Current Health: {}/{}",
                        player.get_health(),
                        player.get_max_health()
                    );
                    std::thread::sleep(std::time::Duration::from_millis(1200));
                } else if item.is_energy_potion() {
                    if player.get_resource() >= player.get_max_resource() {
                        println!("\n⚠️  Your {} is already full!", player.get_resource_name());
                        std::thread::sleep(std::time::Duration::from_millis(1000));
                        continue;
                    }
                    const RESOURCE_RESTORE: u32 = 40;
                    let resource_name = player.get_resource_name().to_string();
                    inventory.remove(index);

                    let current = player.get_resource();
                    let max = player.get_max_resource();
                    let new_resource = (current + RESOURCE_RESTORE).min(max);
                    let actual_restore = new_resource - current;

                    player.set_resource(new_resource);

                    println!(
                        "\n✨ You drink the Energy Potion and restore {} {}!",
                        actual_restore, resource_name
                    );
                    println!(
                        "   Current {}: {}/{}",
                        resource_name,
                        player.get_resource(),
                        player.get_max_resource()
                    );
                    std::thread::sleep(std::time::Duration::from_millis(1200));
                } else {
                    println!("\n📜 {}", item.name());
                    println!("   Value: {} gold", item.value());
                    println!("   This item cannot be used right now.");
                    std::thread::sleep(std::time::Duration::from_millis(1200));
                }
            }
            _ => {
                println!("Invalid choice. Please try again.");
                std::thread::sleep(std::time::Duration::from_millis(800));
            }
        }
    }
}

fn show_and_use_inventory(player: &mut Box<dyn Fighter>, inventory: &mut Vec<Item>) -> bool {
    loop {
        println!("\n┌────────────────────────────────────────┐");
        println!("│  🎒 INVENTORY                          │");
        println!("├────────────────────────────────────────┤");

        if inventory.is_empty() {
            println!("│  (Empty)                               │");
            println!("├────────────────────────────────────────┤");
            println!("│  [0] Back to Combat                    │");
            println!("└────────────────────────────────────────┘");
            print!("\n➤ Choose: ");
            let _ = io::stdout().flush();
            get_user_choice(1);
            return false;
        }

        for (i, item) in inventory.iter().enumerate() {
            println!("│  [{}] {:<30} {}g │", i + 1, item.name(), item.value());
        }

        println!("├────────────────────────────────────────┤");
        println!("│  [0] Back to Combat                    │");
        println!("└────────────────────────────────────────┘");
        print!("\n➤ Choose item to use (or 0 to go back): ");
        let _ = io::stdout().flush();

        let max_choice = inventory.len() as u32;
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        match input.trim().parse::<u32>() {
            Ok(0) => return false, // Back to combat without using item
            Ok(num) if num >= 1 && num <= max_choice => {
                let index = (num - 1) as usize;
                let item = &inventory[index];

                if item.is_health_potion() {
                    const POTION_HEAL: u32 = 30;
                    inventory.remove(index);
                    player.heal(POTION_HEAL);
                    println!("\n✨ You used a Health Potion!");
                    return true; // Item used, end turn
                } else if item.is_energy_potion() {
                    const RESOURCE_RESTORE: u32 = 40;
                    let resource_name = player.get_resource_name().to_string();
                    inventory.remove(index);

                    let current = player.get_resource();
                    let max = player.get_max_resource();
                    let new_resource = (current + RESOURCE_RESTORE).min(max);

                    player.set_resource(new_resource);

                    println!(
                        "\n✨ You used an Energy Potion! Restored {} {}!",
                        new_resource - current,
                        resource_name
                    );
                    return true; // Item used, end turn
                } else {
                    println!("\n⚠️  You can't use that item right now!");
                    continue;
                }
            }
            _ => {
                println!("Invalid choice. Please try again.");
                continue;
            }
        }
    }
}
