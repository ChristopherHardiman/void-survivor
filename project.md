Got it ✅
Here’s a **starter `project.md` file** for your Fyrox **3D Top-Down Arena Survivor Shooter**.
This will serve as your development blueprint — subsystems, milestones, and suggested file structure.

---

```markdown
# 🚀 Project: Fyrox Arena Survivor

A 3D top-down arena survival shooter built with the **Fyrox game engine**.  
Core gameplay: survive waves of enemies, collect loot, level up, and upgrade your ship between rounds.

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
