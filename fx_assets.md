Perfect 👍 — having your FX **organized from the start** will make it way easier to hook into events (weapons, enemy deaths, UI feedback). I’ll rewrite the FX list into a **file-structure oriented format**, with each effect tied to where it lives in `/assets` and how it’s triggered.

Here’s the updated **`fx_asset_list.md`**:

---

```markdown
# ✨ FX Asset List – Fyrox Arena Survivor
Organized by **gameplay category**, with suggested **file locations**.  
All FX are built using **GPU particles, additive blending, and shaders**.  
Goal: lightweight meshes + shader/particle-driven effects.

---

## 📂 File Structure

```

/assets
/particles        # Particle configs
/blaster
/explosions
/loot
/ui
/shaders          # Custom shaders
/weapons
/environment
/ui
/textures         # Noise, masks, glow sprites
/audio            # SFX linked to FX (optional overlap)


## 1. Weapons & Projectiles

**Blaster**
- `/assets/particles/blaster/muzzle_flash.ron` → small burst of sparks at gun tip.  
- `/assets/particles/blaster/projectile.ron` → glowing sphere trail particle.  
- `/assets/particles/blaster/impact_spark.ron` → tiny sparks at hit location.  

**Laser Beam**
- `/assets/shaders/weapons/laser_beam.fx` → scrolling UV + glow.  
- `/assets/particles/blaster/impact_flash.ron` → flash when beam hits target.  

**Rocket / Missile**
- `/assets/particles/explosions/rocket_trail.ron` → flame + smoke exhaust.  
- `/assets/particles/explosions/rocket_explosion.ron` → big burst at impact.  
- `/assets/shaders/weapons/shockwave_ring.fx` → expanding ring on detonation.  

**AoE Pulse / EMP**
- `/assets/shaders/weapons/aoe_ring.fx` → glowing expanding circle.  
- `/assets/particles/explosions/emp_burst.ron` → sparks radiating outward.  
- `/assets/ui/screen_flash.ron` → brief white overlay flash.  

---

## 2. Enemies

**Chaser Drone Death**
- `/assets/particles/explosions/chaser_explode.ron` → small burst + glow.  

**Shooter Drone Death**
- `/assets/particles/explosions/shooter_explode.ron` → larger burst + smoke puff.  

**Tank Drone Death**
- `/assets/particles/explosions/tank_explode.ron` → large explosion fireball.  
- `/assets/shaders/weapons/shockwave_ring.fx` → reuse expanding shockwave.  
- `/assets/particles/explosions/smoke_cloud.ron` → lingering smoke.  

---

## 3. Loot & Pickups

**EXP Orb**
- `/assets/shaders/loot/exp_glow.fx` → pulsing emissive glow.  
- `/assets/particles/loot/exp_orbit.ron` → tiny particles orbiting orb.  

**Health Pack**
- `/assets/shaders/loot/health_glow.fx` → red glow shader.  
- `/assets/particles/loot/health_aura.ron` → faint red mist.  

**Energy Cell**
- `/assets/shaders/loot/energy_glow.fx` → blue glow.  
- `/assets/particles/loot/energy_sparks.ron` → electric sparks.  

**Currency Drop**
- `/assets/shaders/loot/currency_spin.fx` → rotating highlight.  
- `/assets/particles/loot/currency_glow.ron` → faint glow particles.  

---

## 4. Arena & Environment

**Space Background**
- `/assets/shaders/environment/starfield.fx` → twinkling stars.  
- `/assets/shaders/environment/nebula.fx` → perlin/noise cloud scrolling.  
- `/assets/shaders/environment/parallax.fx` → depth layers.  

**Arena Boundary**
- `/assets/shaders/environment/arena_field.fx` → glowing rim shader.  
- `/assets/particles/explosions/boundary_hit.ron` → sparks on collision.  

**Obstacles (Optional)**
- `/assets/particles/environment/asteroid_debris.ron` → dust when hit.  

---

## 5. UI & Feedback

**Player Feedback**
- `/assets/ui/damage_flash.ron` → red screen overlay when hit.  
- `/assets/ui/level_up_burst.ron` → bright particles from player.  

**Shop & Menus**
- `/assets/ui/shop_glow.fx` → pulsing glow on active shop UI.  
- `/assets/ui/wave_start_flash.ron` → radial burst at wave start.  

---

## 🎯 MVP FX Set

✅ Must-have to get to first playable:  
1. `/assets/particles/blaster/projectile.ron` (blaster projectile).  
2. `/assets/particles/explosions/rocket_explosion.ron` (shared for enemies/projectiles).  
3. `/assets/shaders/loot/exp_glow.fx` (EXP orb glow).  
4. `/assets/shaders/environment/starfield.fx` (space background).  
5. `/assets/shaders/environment/arena_field.fx` (arena boundary glow).  

---

---

This way, every FX has a **clear home in `/assets`**, grouped logically:

* **weapons** → `/particles/blaster`, `/shaders/weapons`
* **loot** → `/particles/loot`, `/shaders/loot`
* **environment** → `/shaders/environment`
* **UI** → `/assets/ui`

Would you like me to also create a **roadmap for implementing these FX** (like “start with blaster → add explosions → layer background → add UI feedback”)? That way you’d have a build order to follow.
