Perfect 👌 — going full **space backdrop with shaders** will save you from needing a ton of environmental assets. You’ll just need ships, enemies, items, and some FX meshes.

Here’s a **`model_asset_list.md`** you can drop into your `/docs` folder:

---

```markdown
# 🚀 Model Asset List – Fyrox Arena Survivor

All assets are **3D models** unless noted.  
The arena background is **shader-generated space** (stars, nebula, parallax layers).  
Models should be **low-poly, clean silhouettes**, with shaders/particles providing most of the flair.

---

## 1. Player
- **Player Ship**
  - Small, sleek spacefighter.  
  - Hardpoints for weapons (blasters, lasers, rockets).  
  - Variants: Base ship + possible upgrade skins (optional).  

---

## 2. Enemies
- **Chaser Drone**
  - Spherical or spiky drone that rushes player.  
- **Shooter Drone**
  - Small craft with a turret or forward gun.  
- **Tank Drone**
  - Larger, heavier armored unit.  
  - Slow-moving, maybe with glowing core.  
- **(Optional later)** Elite / Boss Ships
  - Distinct silhouette, multi-weapon.  
  - Larger size for wave climaxes.  

---

## 3. Loot & Pickups
- **EXP Orb**
  - Glowing sphere (small mesh, shader glow).  
- **Health Pack**
  - Small capsule or container with glowing red cross.  
- **Energy Cell (Special Recharge)**
  - Blue/green crystal or capsule with glow.  
- **Currency (Shop)**
  - Floating coin, credit chip, or glowing cube.  

---

## 4. Weapons & Projectiles
(Models may be very simple — particles/shaders do most of the work.)  
- **Blaster Bolt** → small glowing sphere/capsule.  
- **Laser Beam** → line renderer/shader beam (minimal model needed).  
- **Rocket** → small missile mesh with particle trail.  
- **AoE Pulse** → expanding shader ring (no model needed).  
- **Explosion FX** → particle-based (billboards).  

---

## 5. Arena Environment
- **Arena Floor (Optional)**
  - Flat grid or circular arena base.  
  - Subtle glowing borders or force-field edges.  
- **Obstacles (Optional)**
  - Floating asteroids or space debris.  
  - Can double as cover/dynamic obstacles.  

---

## 6. UI / Misc (non-model)
- **Space Background Shader**
  - Starfield with twinkling effect.  
  - Nebula clouds (procedural).  
  - Parallax scrolling.  
- **HUD Icons**
  - Health, shield, EXP, special energy.  
- **Upgrade Icons**
  - Representing ship mods / weapons.  

---

## 7. Stretch Assets (Future Expansion)
- **Boss Ships** (unique silhouettes).  
- **Drones/Pets** (summoned helpers).  
- **Different Arena Skins** (nebula colors, asteroid belts, derelict stations).  
- **Cosmetic Player Skins** (paintjobs, glow effects).  

---

## 🎯 Minimal MVP Asset Set
1. Player Ship  
2. Chaser Drone  
3. Shooter Drone  
4. Tank Drone  
5. EXP Orb  
6. Health Pack  
7. Energy Cell  
8. Blaster Bolt (projectile)  
9. Rocket (projectile)  
10. Arena Base (optional, can use just space background)  

---
```

---
