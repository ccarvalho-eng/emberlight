mod character;
mod enemy;
mod inventory;
mod ui;

use std::io::{self, Write};

use character::{Character, Combat, Fighter, Mage, Rogue, Warrior};
use enemy::Enemy;
use inventory::Item;
use ui::{
    print_banner, print_divider, print_enemy_stats, print_health_bar, print_resource_bar,
    show_character_stats, show_inventory,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    run_game()
}

fn run_game() -> Result<()> {
    print_banner();

    // Character selection
    let mut player_character = select_character();
    player_character.introduce();

    // Give starting items
    println!("\nYou receive a Health Potion!");

    const STARTING_GOLD: u32 = 50;
    const POTION_VALUE: u32 = 50;

    let mut gold = STARTING_GOLD;
    let mut inventory = vec![Item::new(String::from("Health Potion"), POTION_VALUE)];

    println!("\n=== Your Adventure Begins! ===\n");

    // Battle loop
    let mut battle_count = 0;
    loop {
        battle_count += 1;
        print_divider();
        println!("╔════════════════════════════════════════╗");
        println!("║       ENCOUNTER #{:<2}                   ║", battle_count);
        println!("╚════════════════════════════════════════╝");

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
        let reward_gold = 20 + (battle_count * 5);
        gold += reward_gold;
        println!("\n╔════════════════════════════════════════╗");
        println!("║            VICTORY!                    ║");
        println!("╠════════════════════════════════════════╣");
        println!("║  Reward: +{} gold                     ║", reward_gold);
        println!("║  Total Gold: {} coins                 ║", gold);
        println!("╚════════════════════════════════════════╝");

        // Random loot
        if battle_count % 2 == 0 {
            let item = Item::new(String::from("Health Potion"), POTION_VALUE);
            println!("\n🎁 You found a {}!", item.name());
            inventory.push(item);
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
                    show_inventory(&inventory, gold);
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
    match encounter {
        1 => Enemy::goblin(1),
        2 => Enemy::goblin(2),
        3 => Enemy::orc(2),
        4 => Enemy::orc(3),
        _ => {
            let level = if encounter > 2 { encounter - 2 } else { 1 };
            Enemy::dragon(level)
        }
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
                let damage = player.attack();
                println!("\n💥 You attack for {} damage!", damage);
                enemy.take_damage(damage);
            }
            2 => {
                if let Some(damage) = player.special_ability() {
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
        let damage = enemy.attack();
        println!("💥 {} attacks for {} damage!", enemy.get_name(), damage);
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

                match item.name() {
                    "Health Potion" => {
                        const POTION_HEAL: u32 = 30;
                        inventory.remove(index);
                        player.heal(POTION_HEAL);
                        println!("\n✨ You used a Health Potion!");
                        return true; // Item used, end turn
                    }
                    _ => {
                        println!("\n⚠️  You can't use that item right now!");
                        continue;
                    }
                }
            }
            _ => {
                println!("Invalid choice. Please try again.");
                continue;
            }
        }
    }
}
