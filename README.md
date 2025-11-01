# 🔥 Emberlight

A dark fantasy turn-based RPG built with Rust, demonstrating advanced trait systems, resource management, and interactive combat mechanics.

## ✨ Features

### Combat System
- **Resource Management**: Each class has unique resources (Mana, Rage, Energy)
- **Special Abilities**: Powerful skills with resource costs and strategic timing
- **Dynamic Combat Menu**: Real-time display of available abilities and costs
- **Interactive Inventory**: Use items during combat or from the inventory menu

### Character Classes

#### ⚔️ Warrior - Tank & Heavy Hitter
- **Resource**: Rage (50 + 5/level)
- **Special**: Shield Bash - Deals 2x attack damage (Costs 20 Rage)
- High HP and armor for survivability

#### 🔮 Mage - Spell Caster
- **Resource**: Mana (80 + 10/level)
- **Special**: Fireball - Deals 3x intelligence damage (Costs 30 Mana)
- Powerful spells but lower health

#### 🗡️ Rogue - Swift Assassin
- **Resource**: Energy (100 + 10/level)
- **Special**: Backstab - Critical strike for 3x damage (Costs 40 Energy)
- Balanced stats with high burst damage

## 🎮 How to Play

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/emberlight.git
cd emberlight

# Build and run
cargo run --release
```

### Controls

**Character Selection**
- Choose your class: Warrior, Mage, or Rogue
- Enter your character name

**Combat**
- `[1]` Attack - Basic attack with no resource cost
- `[2]` Special - Use your class's special ability (if you have enough resources)
- `[3]` Potion - Quick-use a Health Potion
- `[4]` Inventory - Open interactive inventory to use items

**Main Menu (Between Battles)**
- `[1]` Continue Quest - Face the next enemy
- `[2]` Stats - View your character statistics
- `[3]` Inventory - View your items and gold
- `[4]` Rest & Quit - End your adventure

## 🏗️ Architecture

### Trait-Based Design
```rust
pub trait Character { /* Health, Level, Name */ }
pub trait ResourcePool { /* Mana/Rage/Energy management */ }
pub trait Combat { /* Attack, Special abilities */ }
pub trait Fighter: Character + Combat + ResourcePool {}
```

### Module Structure
```
src/
├── main.rs           # Game loop and core logic
├── character/        # Character traits and classes
│   ├── mod.rs
│   ├── traits.rs     # Trait definitions
│   ├── warrior.rs
│   ├── mage.rs
│   └── rogue.rs
├── enemy.rs          # Enemy types and AI
├── inventory/        # Items and inventory system
│   ├── mod.rs
│   └── item.rs
└── ui.rs             # Display and UI utilities
```

## 🎯 Game Mechanics

### Resource Management
- Resources are **consumed** when using special abilities
- Strategic decision: Save resources for powerful attacks or use basic attacks?
- Different resource pools encourage unique playstyles

### Combat Flow
1. Player chooses action (Attack, Special, Potion, Inventory)
2. Damage is calculated and applied
3. Check for enemy defeat
4. Enemy counterattacks
5. Check for player defeat
6. Repeat

### Progression
- Defeat enemies to earn gold
- Find Health Potions after even-numbered encounters
- Face increasingly difficult enemies (Goblins → Orcs → Dragons)
- Survive as many encounters as possible

## 🛠️ Technical Features

### Rust Best Practices
- ✅ Proper error handling with `Result` types
- ✅ Trait objects for polymorphism
- ✅ Private fields with accessor methods
- ✅ Const values for magic numbers
- ✅ Derive macros (Debug, Clone, PartialEq)
- ✅ Zero clippy warnings
- ✅ Formatted with rustfmt

### Code Quality
- Clean separation of concerns (UI, Combat, Character, Inventory)
- Type-safe resource management
- Extensible item system
- Well-documented code structure

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Feel free to:
- Report bugs
- Suggest new features
- Add new character classes
- Implement new items and abilities
- Improve the UI/UX

## 🎨 Future Enhancements

- [ ] More character classes (Paladin, Ranger, etc.)
- [ ] Equipment system (Weapons, Armor)
- [ ] Skill trees and leveling
- [ ] Save/Load game functionality
- [ ] Multiple enemy types per encounter
- [ ] Boss battles with special mechanics
- [ ] Story mode with narrative elements
- [ ] Multiplayer arena mode

## 💡 Learning Resources

This project demonstrates:
- Trait-based polymorphism in Rust
- Resource management patterns
- Interactive CLI application design
- Modular code organization
- State management in game loops

Perfect for learning Rust through game development!

---

Built with ❤️ using Rust