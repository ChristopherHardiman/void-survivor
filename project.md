Got it ✅
Here’s a **starter `project.md` file** for your Fyrox **3D Top-Down Arena Survivor Shooter**.
This will serve as your development blueprint — subsystems, milestones, and suggested file structure.

---

```markdown
# Bevy Arena Survivor

## Project Overview
**Bevy Arena Survivor** is a 3D top-down arena survival shooter built with the Bevy game engine. Players control a spaceship in a confined arena, fighting waves of increasingly difficult enemies while collecting loot and upgrading their abilities.

## Game Concept

### Core Gameplay Loop
1. **Survive Waves**: Fight off waves of enemies with different behaviors and abilities
2. **Collect Loot**: Gather experience points, health, shields, and power-ups
3. **Level Up**: Gain experience to unlock new abilities and weapon upgrades
4. **Upgrade Arsenal**: Choose from various weapon types and special abilities
5. **Endless Challenge**: Survive as long as possible with escalating difficulty

### Key Features
- **Dynamic Combat**: Fast-paced shooting with multiple weapon types
- **Enemy Variety**: 6+ distinct enemy types with unique AI behaviors
- **Progression System**: Level-based upgrades and weapon unlocks
- **Visual Effects**: Particle systems, screen shake, and visual feedback
- **Adaptive Difficulty**: Waves scale in complexity and challenge
- **Loot System**: Various pickups to enhance player capabilities

## Technical Architecture

### Engine: Bevy Game Engine 0.12
- **ECS Architecture**: Entity-Component-System for performant game logic
- **Cross-Platform**: Native support for Windows, macOS, Linux, and Web
- **2D/3D Rendering**: Flexible rendering pipeline with sprite and mesh support
- **Audio System**: Built-in spatial audio and music management
- **Asset Pipeline**: Efficient loading and management of game resources

### Project Structure
```
void_survivor/
├── game/                    # Main game library
│   ├── src/
│   │   ├── main.rs         # Application entry point
│   │   ├── lib.rs          # Library interface
│   │   ├── player.rs       # Player systems and components
│   │   ├── enemy.rs        # Enemy AI and behavior
│   │   ├── wave.rs         # Wave spawning and progression
│   │   ├── loot.rs         # Loot drops and collection
│   │   ├── upgrade.rs      # Player progression system
│   │   ├── ui.rs           # User interface and HUD
│   │   ├── fx.rs           # Particle effects and visuals
│   │   ├── audio.rs        # Sound effects and music
│   │   └── config.rs       # Game balance and settings
│   └── Cargo.toml          # Dependencies and build config
├── assets/                  # Game assets
│   ├── sprites/            # 2D textures and images
│   ├── models/             # 3D models (future)
│   ├── audio/              # Sound effects and music
│   ├── fonts/              # UI fonts
│   └── shaders/            # Custom shaders (future)
└── config/                  # Configuration files
    ├── game_config.ron     # Game balance settings
    ├── player_config.ron   # Player stats configuration
    ├── enemy_config.ron    # Enemy behavior settings
    └── wave_config.ron     # Wave progression rules
```

## Game Systems

### Player System
- **Movement**: WASD/Arrow keys for 8-directional movement
- **Combat**: Mouse aiming with left-click or spacebar shooting
- **Health/Shields**: Dual-layer defense system with regeneration
- **Weapons**: Multiple weapon types with unique characteristics
- **Experience**: Level-based progression with upgrade choices

### Enemy System
- **Chaser**: Basic melee enemy that pursues the player
- **Shooter**: Ranged enemy that fires projectiles at player
- **Tank**: Heavy armored enemy with high health and damage
- **Swarm**: Fast, weak enemies that attack in groups
- **Elite**: Advanced enemies with special abilities
- **Boss**: Large enemies with complex attack patterns

### Wave System
- **Dynamic Spawning**: Enemies spawn around arena perimeter
- **Difficulty Scaling**: Health, damage, and count increase per wave
- **Enemy Composition**: Different enemy mixes based on wave number
- **Break Periods**: Short rest between waves for strategy
- **Boss Waves**: Special waves featuring boss enemies

### Loot System
- **Experience Orbs**: Primary progression currency
- **Health Pickups**: Restore player health
- **Shield Boosters**: Restore and upgrade shields
- **Weapon Drops**: Unlock new weapon types
- **Power-ups**: Temporary ability enhancements

### Upgrade System
- **Level Up Choices**: Select from 3 random upgrades per level
- **Stat Improvements**: Health, damage, speed, fire rate boosts
- **Weapon Unlocks**: Access to new weapon types
- **Special Abilities**: Unique powers like double shot, piercing
- **Tier Progression**: Upgrades scale with player level

## Art and Audio Direction

### Visual Style
- **Top-Down Perspective**: Clear view of arena and all entities
- **Space Theme**: Sci-fi aesthetic with ships and energy weapons
- **Particle Effects**: Explosions, muzzle flashes, and impact effects
- **UI Design**: Clean, futuristic interface with clear information
- **Color Coding**: Visual indicators for different enemy types

### Audio Design
- **Dynamic Music**: Adaptive soundtrack that responds to gameplay
- **Sound Effects**: Satisfying audio feedback for all actions
- **Spatial Audio**: 3D positioned sounds for immersion
- **Audio Scaling**: Sound intensity matches game tension

## Development Roadmap

### Phase 1: Core Systems (Current)
- [x] Basic Bevy project setup
- [x] Player movement and shooting
- [x] Enemy spawning and AI
- [x] Wave progression system
- [x] Basic UI and HUD
- [ ] Loot collection
- [ ] Upgrade system

### Phase 2: Polish and Balance
- [ ] Particle effects and visual feedback
- [ ] Audio implementation
- [ ] Game balance tuning
- [ ] Menu systems
- [ ] Save/load functionality

### Phase 3: Content Expansion
- [ ] Additional enemy types
- [ ] More weapon varieties
- [ ] Special abilities system
- [ ] Boss encounters
- [ ] Achievement system

### Phase 4: Platform Support
- [ ] Web deployment (WASM)
- [ ] Mobile adaptation
- [ ] Performance optimization
- [ ] Release preparation

## Technical Requirements

### Dependencies
- **Bevy 0.12**: Core game engine
- **Serde**: Configuration serialization
- **Ron**: Rusty Object Notation for configs
- **Rand**: Random number generation

### Build Targets
- **Native**: Windows, macOS, Linux desktop
- **Web**: WebAssembly browser deployment
- **Mobile**: iOS and Android (future consideration)

### Performance Goals
- **60 FPS**: Stable frame rate on target hardware
- **Low Latency**: Responsive input handling
- **Memory Efficient**: Reasonable resource usage
- **Fast Loading**: Quick startup and asset loading

## Configuration System

The game uses Ron (Rusty Object Notation) configuration files for easy tweaking of game balance:

- **game_config.ron**: Arena size, difficulty scaling, loot rates
- **player_config.ron**: Base stats, movement speed, weapon stats
- **enemy_config.ron**: Enemy health, damage, AI behavior parameters
- **wave_config.ron**: Spawn rates, wave composition, break timings

This allows for rapid iteration and balance adjustments without recompiling the game.

## Future Considerations

### Potential Enhancements
- **Multiplayer Support**: Co-op survival mode
- **Level Editor**: Custom arena creation
- **Mod Support**: Community content creation
- **Statistics Tracking**: Detailed gameplay analytics
- **Leaderboards**: Global and local high scores

### Technical Improvements
- **Advanced AI**: State machines for complex enemy behavior
- **Physics Integration**: Bevy Rapier for realistic collision
- **Shader Effects**: Custom visual effects and post-processing
- **Networking**: Foundation for multiplayer features

This project serves as both an engaging game and a learning platform for Bevy game engine development, demonstrating modern Rust game programming techniques and ECS architecture patterns.

---

## 🎮 Core Gameplay Loop
1. Player fights through enemy waves.  
2. Enemies drop EXP, health, and special recharge items.  
3. Player can **pause wave manually** → access level-up screen (choose 1 upgrade).  
4. Between waves → **shop phase** → buy ship upgrades/weapons.  
5. Waves get progressively harder and longer.  

---

## 🧩 Subsystems

### 1. Player System
- Movement: WASD + mouse aim (twin-stick style).  
- Weapons: shader/particle driven (blaster, laser, rockets, AoE pulse).  
- Stats: health, shields, special energy, exp.  
- Level-up: triggered by EXP threshold or player pausing the wave.  

### 2. Enemy System
- Types:  
  - Chaser (melee).  
  - Shooter (ranged).  
  - Tank (slow, high HP).  
- Spawning managed by **Wave Manager**.  
- Drops loot: EXP orbs, health packs, special energy.  

### 3. Wave Manager
- Controls wave spawning, timers, and scaling difficulty.  
- Handles "pause wave" mechanic.  
- Notifies UI when wave ends (transition to shop).  

### 4. Upgrade & Level-Up System
- **Mid-wave level-up**:  
  - Pauses wave, shows UI with 2–3 random upgrades.  
- **Upgrade types**:  
  - Weapon upgrades (damage, fire rate, projectile count).  
  - Ship upgrades (speed, armor, shield recharge).  
  - Special upgrades (area attacks, drones, energy regen).  

### 5. Shop System (Between Waves)
- Uses currency dropped from enemies.  
- Purchasable items:  
  - New weapons.  
  - Ship upgrades.  
  - Special abilities.  
- UI menu with item name, description, and cost.  

### 6. Loot & Items
- EXP Orbs → increase level bar.  
- Health Packs → restore HP.  
- Energy Cells → recharge special attacks.  

### 7. UI System
- **HUD**:  
  - Health bar, shield bar, EXP bar, wave timer.  
- **Overlays**:  
  - Level-up screen (choice UI).  
  - Shop screen (between waves).  
- **Menu system**: Start game, settings, quit.  

### 8. Visual Effects
- Weapons use **shaders + particles**:  
  - Bullets: glowing particles.  
  - Lasers: custom shader beam.  
  - Explosions: GPU particle burst.  
- Arena visuals:  
  - Simple floor/arena mesh.  
  - Ambient lighting.  
  - Minimal obstacles.  

### 9. Audio
- Background music loop.  
- Weapon fire sounds.  
- Explosion + impact FX.  
- UI clicks + shop sounds.  

---

## 📂 Suggested Project Structure

```

/src
main.rs                # Game entry point
game.rs                # Game state management
player.rs              # Player logic + weapons
enemy.rs               # Enemy AI + spawning
wave.rs                # Wave manager system
loot.rs                # Loot drops + pickups
ui.rs                  # HUD, menus, level-up, shop
upgrade.rs             # Upgrade + shop logic
fx.rs                  # Shaders + particle systems
audio.rs               # Sound manager
config.rs              # Balancing + constants

/assets
/models                # Player + enemy 3D models
/textures              # Arena + item textures
/particles             # Particle definitions
/shaders               # Custom weapon/beam shaders
/audio                 # Music + sound effects
/ui                    # UI layout + assets

/docs
project.md             # This design doc
roadmap.md             # Development milestones


---

## 🛠️ Development Milestones

### Phase 1 – Core Systems
- [ ] Player movement + shooting (basic weapon).  
- [ ] Basic enemy AI (chaser).  
- [ ] Wave Manager spawns enemies.  
- [ ] Collisions + damage system.  

### Phase 2 – Loot & Progression
- [ ] Add EXP + leveling system.  
- [ ] Implement level-up screen (random upgrades).  
- [ ] Add health + special energy drops.  

### Phase 3 – Shop Phase
- [ ] End-wave shop screen.  
- [ ] Currency system.  
- [ ] Purchasable upgrades + new weapons.  

### Phase 4 – Visual & Audio Polish
- [ ] Shader/particle effects for weapons.  
- [ ] Arena visuals + lighting.  
- [ ] Sound effects + background music.  

### Phase 5 – Advanced Features (Stretch)
- [ ] More enemy types (shooters, tanks, elites).  
- [ ] Boss waves every X levels.  
- [ ] Multiple arenas.  
- [ ] Randomized shop offers (roguelike style).  

---

## 🎯 MVP Goal
A playable loop:  
- Player survives at least **5 progressively harder waves**.  
- EXP + level-up system works.  
- Shop offers upgrades between waves.  
- Weapons use **basic particles/shaders** for effects.  

---


---
