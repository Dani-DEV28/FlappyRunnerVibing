🦾 Flap Runner (Working Title)

A 2D side-scrolling action runner built with Rust and the Bevy Game Engine, inspired by Jetpack Joyride-style traversal, River City Girls-like presentation, and featuring a mutated buff humanoid flappy protagonist navigating chaotic obstacle environments.

🎯 Core Concept

The player controls a powerful mutated flappy humanoid character equipped with jet-assisted flight mechanics. The gameplay centers around continuous forward movement, vertical control (flap/boost), and survival through increasingly dense obstacle patterns.

🧱 MVP Roadmap

The MVP is intentionally minimal: movement → camera → collision → expansion hooks

1. Player Movement (Core Interaction)

Goal: Get a controllable character moving on a 2D plane.

Initialize Bevy project with a 2D camera
Create a player entity (sprite or placeholder shape)
Implement basic physics-like movement:
Gravity-like downward force
“Flap” or jetpack impulse upward
Horizontal auto-scroll or manual movement (decide early)
Ensure frame-rate independent movement (use delta time)

Definition of Done:

Player can move vertically via input
Player is affected by gravity/constant downward force
Movement feels responsive and stable
2. Camera System (Follow + Lock)

Goal: Keep player centered in view during gameplay.

Add a 2D camera entity
Implement camera follow system:
Smooth follow OR hard lock (start with hard lock for MVP)
Optionally constrain camera to X-axis scrolling only

Definition of Done:

Camera consistently tracks player position
No jitter or desync between player and viewport
World scrolls relative to player movement
3. Collision System (Basic Obstacles)

Goal: Introduce physical interaction with the environment.

Add simple obstacle entities (rectangles or sprites)
Implement bounding box collision (AABB is sufficient for MVP)
On collision:
Stop movement OR reset player position OR print debug state
Structure collision logic so it can later expand into damage, knockback, etc.

Definition of Done:

Player detects collisions reliably
Obstacles affect gameplay state in at least one meaningful way
Collision system is modular (not hard-coded into movement logic)
🧪 Future Work (TBD)

Once MVP is stable, expand into full gameplay systems:

Movement Enhancements
Jetpack fuel / stamina system
Acceleration curves (less linear movement)
Dash / burst mechanics
Gameplay Systems
Procedural obstacle spawning
Enemy entities with simple AI
Power-ups (shield, speed boost, magnet, etc.)
Visual & Feel
Animated protagonist states (idle, flap, hurt)
Parallax background layers
Screen shake on collision
Particle effects for jet bursts
Progression
Distance-based scoring system
Increasing difficulty over time
Unlockable abilities or character mutations
🛠 Tech Stack
Rust
Bevy Engine
2D Coordinate System (orthographic camera)
ECS-based architecture (Bevy default)
🧠 Design Philosophy
Keep MVP brutally simple: movement first, everything else later
Prefer readable systems over clever abstractions early
Treat gameplay feel as a first-class feature, not polish
Build in “test hooks” (debug input, collision toggles, etc.)
🚀 MVP Milestone Order
Player movement (gravity + flap)
Camera follow system
Collision with basic obstacles
Expand systems only after stability
