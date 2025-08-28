# Fyrox Arena Survivor

A 3D top-down arena survival shooter built with the Fyrox game engine.

## Project Structure

### Game Modules (`game/src/`)
- **`player.rs`** - Player movement, weapons, stats, and controls
- **`enemy.rs`** - Enemy AI, types (Chaser, Shooter, Tank), and behaviors  
- **`wave.rs`** - Wave management, spawning, difficulty scaling
- **`loot.rs`** - Experience orbs, health packs, energy cells, currency
- **`upgrade.rs`** - Level-up system, shop items, player progression
- **`ui.rs`** - HUD, menus, level-up screen, shop interface
- **`fx.rs`** - Particle systems, visual effects, shaders
- **`audio.rs`** - Sound effects, music, audio management
- **`config.rs`** - Game balance, settings, configuration system

### Assets Structure (`assets/`)
```
assets/
├── models/          # 3D models for ships, enemies, items
├── textures/        # Textures for models and effects
├── particles/       # Particle system configurations
│   ├── blaster/     # Blaster weapon effects
│   ├── explosions/  # Enemy death and impact effects
│   ├── loot/        # Loot glow and attraction effects
│   └── environment/ # Arena and background effects
├── shaders/         # Custom shaders
│   ├── weapons/     # Laser beams, shockwaves, AoE rings
│   ├── loot/        # Glowing item effects
│   └── environment/ # Starfield, nebula, arena boundary
├── audio/           # Sound effects and music
└── ui/              # UI assets and screen effects
```

### Executors
- **`executor/`** - Desktop executable
- **`executor-wasm/`** - Web build target
- **`executor-android/`** - Android build target
- **`editor/`** - Fyrox editor integration

## Current Status

✅ **Completed:**
- Base project structure and Cargo workspace setup
- Core game system modules defined
- Asset directory structure organized
- Configuration system with balance parameters
- Development roadmap created

🔧 **Next Steps (Phase 1):**
1. Fix Fyrox Plugin trait implementation
2. Create basic 3D arena scene
3. Implement player movement and shooting
4. Add basic enemy spawning and AI

## Core Gameplay Concept

**Survival Loop:**
1. Fight waves of enemies (Chasers, Shooters, Tanks)
2. Collect EXP, health, and currency drops
3. Level up mid-wave with upgrade choices
4. Shop between waves for new weapons and abilities
5. Survive progressively harder waves

**Key Features:**
- **Twin-stick style controls** - WASD movement + mouse aim
- **Multiple weapons** - Blaster, Laser, Rockets, AoE Pulse
- **Manual wave pausing** - Choose when to level up
- **Shop system** - Buy upgrades between waves
- **Shader-driven effects** - Particles and glowing visuals

## Development Timeline

- **Phases 1-2 (Weeks 1-4):** Core gameplay loop
- **Phase 3 (Weeks 5-6):** Shop system and weapon variety  
- **Phase 4 (Weeks 7-8):** Visual and audio polish
- **Phases 5-6 (Weeks 9-12):** Advanced features and final polish

See `roadmap.md` for detailed development phases and tasks.

## Quick Start

1. **Build the project:**
   ```bash
   cargo build
   ```

2. **Run the editor:**
   ```bash
   cargo run --bin editor
   ```

3. **Run the game:**
   ```bash
   cargo run --bin executor
   ```

## Asset Requirements (MVP)

**Essential 3D Models:**
- Player ship (small fighter)
- 3x Enemy drones (Chaser, Shooter, Tank)
- Loot items (EXP orb, health pack, energy cell)
- Basic projectiles (blaster bolt, rocket)

**Essential Effects:**
- Weapon muzzle flashes and trails
- Enemy death explosions  
- Loot glow shaders
- Arena boundary visual

**Essential Audio:**
- Weapon firing sounds
- Enemy death sounds
- Loot pickup audio
- Background music loop

## Design Philosophy

- **"Juice over realism"** - Focus on satisfying visual/audio feedback
- **Shader-driven effects** - Minimize complex 3D assets, maximize shaders
- **Simple but deep** - Easy to learn, depth through upgrade combinations
- **Incremental development** - Build, test, iterate quickly

The goal is to create a highly polished, focused arcade experience that feels great to play.
