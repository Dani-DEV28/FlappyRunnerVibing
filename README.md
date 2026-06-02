# 🦾 Flappy Puncher (Working Title)

A 2D side-scrolling beat 'em up built with Rust and the Bevy Game Engine, featuring a mutated, buff humanoid protagonist who punches, combos, and brawls through hordes of enemies and environmental hazards.

---

## 🎯 Core Concept

The player controls a powerful mutated humanoid in a classic side-scrolling beat 'em up format. Gameplay focuses on responsive grounded movement, melee attacks, combos, simple special moves, and tactical use of the environment to clear waves of enemies and progress through stages.

---

## 🧱 MVP Roadmap

The MVP emphasizes core combat systems first: movement → attacks → enemies → hit detection → camera

---

### 1. Player Movement & Controls (Core Interaction)

**Goal:** Deliver tight left/right movement with jump and facing direction.

- Initialize Bevy project with a 2D orthographic camera
- Create a player entity (sprite or placeholder) with directional facing
- Implement basic movement inputs:
  - Move left / move right (frame-rate independent)
  - Short jump (optional for vertical gameplay)
  - Facing flip based on movement or aim

**Definition of Done:**

- Player can move left and right responsively
- Player faces the correct direction when moving or attacking
- Movement is consistent across frame rates

---

### 2. Melee Combat (Attacks & Combos)

**Goal:** Add a simple attack system with an attack animation window and combo chaining.

- Implement a basic melee attack with hitbox spawn timing
- Support a lightweight combo chain (2-3 hits) with timing windows
- Add knockback and brief invulnerability on hit

**Definition of Done:**

- Player can perform primary attacks
- Combos register sequential hits when timed correctly
- Enemies react to hit with knockback or stagger

---

### 3. Enemies & AI

**Goal:** Add simple enemy entities that approach and attack the player.

- Create a basic enemy archetype with health and patrol/aggro behavior
- Implement simple attack telegraphs and hitboxes
- Spawn a small wave to test combat flow

**Definition of Done:**

- Enemies move toward the player and can be damaged
- Enemy attacks can hit the player and cause damage/knockback

---

### 4. Hit Detection & Damage

**Goal:** Reliable collision/hit detection between attack hitboxes and entities.

- Use AABB or simple shape overlap for hitboxes
- Implement health, damage application, and death state
- Separate collision for environment vs. combat

**Definition of Done:**

- Attacks consistently register hits
- Damage and health updates are reflected in game state

---

## 🧪 Future Work (TBD)

Once core combat is stable, expand into richer beat 'em up systems:

### Combat Enhancements
- Special moves and charge attacks
- Grab/throw mechanics and environmental interactions
- Combo meters or stun systems

### Gameplay Systems
- Stage-based enemy waves and checkpoints
- Boss encounters with telegraphed phases
- Power-ups and pickups (health, temporary buffs)

### Visual & Feel
- Animated attack states (idle, attack, stagger, death)
- Parallax backgrounds and screen shake on heavy hits
- Particle and sound effects for impact

---

## 🛠 Tech Stack

- Rust
- Bevy Engine
- 2D Coordinate System (orthographic camera)
- ECS-based architecture (Bevy default)

---

## 🧠 Design Philosophy

- Prioritize combat feel and responsiveness over feature breadth
- Keep systems modular so combat, AI, and movement can be iterated independently
- Provide debug hooks for spawning enemies, tweaking hitboxes, and toggling invulnerability

---

## 🚀 MVP Milestone Order

- Player movement and facing
- Primary melee attack and combo window
- Basic enemy archetype and simple AI
- Hit detection and health/damage systems
- Iterate on combat feel and camera behavior