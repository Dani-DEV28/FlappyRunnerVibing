# Flappy Puncher - Project Summary

## Project Structure

```
FlappyRunnerVibing/
├── Cargo.toml          # Bevy 0.15.3, dev optimizations
├── .gitignore
└── src/
    ├── main.rs         # App setup, 2D camera, ground platform
    ├── player.rs       # Movement (A/D/arrows), jump (Space), gravity, facing
    ├── combat.rs       # Melee attacks (J/Enter), 3-hit combo chain, hitbox spawning
    ├── enemy.rs        # 5 enemies with patrol/aggro AI, attack cooldown
    ├── damage.rs       # AABB hit detection, health, knockback, death/despawn
    └── camera.rs       # Smooth X-axis follow on player
```

## Controls

| Key | Action |
|-----|--------|
| A/D or ←/→ | Move left/right |
| Space | Jump |
| J or Enter | Attack (chain up to 3 hits) |

## How to Run

```
cargo run
```

## Gameplay

- Green rectangle = player
- Red rectangles = enemies
- Enemies aggro within 250px and attack when close
- Combo hits deal increasing damage (10/15/20) with increasing knockback
- Enemies die after taking enough damage and despawn on death
