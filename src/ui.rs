use crate::character::Character;
use crate::enemy::Enemy;
use crate::inventory::Item;

pub fn print_banner() {
    println!("\n");
    println!(" ███████╗███╗   ███╗██████╗ ███████╗██████╗ ██╗     ██╗ ██████╗ ██╗  ██╗████████╗");
    println!(" ██╔════╝████╗ ████║██╔══██╗██╔════╝██╔══██╗██║     ██║██╔════╝ ██║  ██║╚══██╔══╝");
    println!(" █████╗  ██╔████╔██║██████╔╝█████╗  ██████╔╝██║     ██║██║  ███╗███████║   ██║   ");
    println!(" ██╔══╝  ██║╚██╔╝██║██╔══██╗██╔══╝  ██╔══██╗██║     ██║██║   ██║██╔══██║   ██║   ");
    println!(" ███████╗██║ ╚═╝ ██║██████╔╝███████╗██║  ██║███████╗██║╚██████╔╝██║  ██║   ██║   ");
    println!(" ╚══════╝╚═╝     ╚═╝╚═════╝ ╚══════╝╚═╝  ╚═╝╚══════╝╚═╝ ╚═════╝ ╚═╝  ╚═╝   ╚═╝   ");
    println!(
        "                       🔥 Dark Fantasy Turn-Based RPG 🔥                          \n"
    );
}

pub fn print_divider() {
    println!("\n{}", "═".repeat(42));
}

pub fn print_health_bar(label: &str, current: u32, max: u32) {
    let percentage = (current as f32 / max as f32 * 100.0) as u32;
    let bar_length = 20;
    let filled = (current as f32 / max as f32 * bar_length as f32) as usize;
    let empty = bar_length - filled;

    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));
    let hp_text = format!("{}/{}", current, max);

    println!("│  {:>6}: {} {:>8}  {}% │", label, bar, hp_text, percentage);
}

pub fn print_enemy_stats(enemy: &Enemy) {
    println!("\n┌────────────────────────────────────────┐");
    println!(
        "│  Level: {}  │  HP: {}/{}  ",
        enemy.get_level(),
        enemy.get_health(),
        enemy.get_max_health()
    );
    println!("└────────────────────────────────────────┘");
}

pub fn show_inventory(inventory: &[Item], gold: u32) {
    println!("\n┌────────────────────────────────────────┐");
    println!("│  🎒 INVENTORY                          │");
    println!("├────────────────────────────────────────┤");
    println!("│  💰 Gold: {} coins                    │", gold);
    println!("├────────────────────────────────────────┤");

    if inventory.is_empty() {
        println!("│  (Empty)                               │");
    } else {
        for (i, item) in inventory.iter().enumerate() {
            println!("│  {}. {:<30} {}g │", i + 1, item.name(), item.value());
        }
    }
    println!("└────────────────────────────────────────┘");
}

pub fn print_resource_bar(label: &str, current: u32, max: u32) {
    let percentage = (current as f32 / max as f32 * 100.0) as u32;
    let bar_length = 20;
    let filled = (current as f32 / max as f32 * bar_length as f32) as usize;
    let empty = bar_length - filled;

    let bar = format!("[{}{}]", "▓".repeat(filled), "░".repeat(empty));
    let resource_text = format!("{}/{}", current, max);

    println!(
        "│  {:>6}: {} {:>8}  {}% │",
        label, bar, resource_text, percentage
    );
}

pub fn show_character_stats(character: &dyn Character, gold: u32, battles_won: u32) {
    println!("│  Name: {:<32} │", character.get_name());
    println!("│  Level: {:<31} │", character.get_level());
    print_health_bar("HP", character.get_health(), character.get_max_health());
    println!("│  Gold: {:<32} │", gold);
    println!("│  Battles Won: {:<26} │", battles_won);
}
